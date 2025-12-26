
// Actual definitions from macro_matching.rs
pub fn try_match_macro<'matcher, T: Tracker<'matcher>>(
    psess: &ParseSess,
    name: Ident,
    arg: &TokenStream,
    rules: &'matcher [MacroRule],
    track: &mut T,
) -> Result<(usize, &'matcher MacroRule, mbe_macro_parser::NamedMatches), CanRetry> {
    // We create a base parser that can be used for the "black box" parts.
    // Every iteration needs a fresh copy of that parser. However, the parser
    // is not mutated on many of the iterations, particularly when dealing with
    // macros like this:
    //
    // macro_rules! foo {
    //     ("a") => (A);
    //     ("b") => (B);
    //     ("c") => (C);
    //     // ... etc. (maybe hundreds more)
    // }
    //
    // as seen in the `html5ever` benchmark. We use a `Cow` so that the base
    // parser is only cloned when necessary (upon mutation). Furthermore, we
    // reinitialize the `Cow` with the base parser at the start of every
    // iteration, so that any mutated parsers are not reused. This is all quite
    // hacky, but speeds up the `html5ever` benchmark significantly. (Issue
    // 68836 suggests a more comprehensive but more complex change to deal with
    // this situation.)
    let parser = macro_rules_utils::parser_from_cx(psess, arg.clone(), T::recovery());
    // Try each arm's matchers.
    let mut tt_parser = mbe_macro_parser::TtParser::new(name);
    for (i, rule) in rules.iter().enumerate() {
        let MacroRule::Func { lhs, .. } = rule else { continue };
        let _tracing_span = trace_span!("Matching arm", %i);

        // Take a snapshot of the state of pre-expansion gating at this point.
        // This is used so that if a matcher is not `ParseResult::Success(..)`ful,
        // then the spans which became gated when parsing the unsuccessful matcher
        // are not recorded. On the first `ParseResult::Success(..)`ful matcher, the spans are merged.
        let mut gated_spans_snapshot = mem::take(&mut *psess.gated_spans.spans.borrow_mut());

        let result = tt_parser.parse_tt(&mut Cow::Borrowed(&parser), lhs, track);

        track.after_arm(true, &result);

        match result {
            mbe_macro_parser::ParseResult::Success(named_matches) => {
                debug!("Parsed arm successfully");
                // The matcher was `ParseResult::Success(..)`ful.
                // Merge the gated spans from parsing the matcher with the preexisting ones.
                psess.gated_spans.merge(gated_spans_snapshot);

                return Ok((i, rule, named_matches));
            }
            mbe_macro_parser::ParseResult::Failure(_) => {
                trace!("Failed to match arm, trying the next one");
                // Try the next arm.
            }
            mbe_macro_parser::ParseResult::Error(_, _) => {
                debug!("Fatal error occurred during matching");
                // We haven't emitted an error yet, so we can retry.
                return Err(CanRetry::Yes);
            }
            mbe_macro_parser::ParseResult::ErrorReported(guarantee) => {
                debug!("Fatal error occurred and was reported during matching");
                // An error has been reported already, we cannot retry as that would cause duplicate errors.
                return Err(CanRetry::No(guarantee));
            }
        }

        // The matcher was not `ParseResult::Success(..)`ful.
        // Restore to the state before snapshotting and maybe try again.
        mem::swap(&mut gated_spans_snapshot, &mut psess.gated_spans.spans.borrow_mut());
    }

    Err(CanRetry::Yes)
}
