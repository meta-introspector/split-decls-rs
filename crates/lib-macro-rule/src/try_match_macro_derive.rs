pub fn try_match_macro_derive<'matcher, T: Tracker<'matcher>>(
    psess: &ParseSess,
    name: Ident,
    body: &TokenStream,
    rules: &'matcher [MacroRule],
    track: &mut T,
) -> Result<(usize, &'matcher MacroRule, mbe_macro_parser::NamedMatches), CanRetry> {
    // This uses the same strategy as `try_match_macro`
    let body_parser = macro_rules_utils::parser_from_cx(psess, body.clone(), T::recovery());
    let mut tt_parser = mbe_macro_parser::TtParser::new(name);
    for (i, rule) in rules.iter().enumerate() {
        let MacroRule::Derive { body, .. } = rule else { continue };

        let mut gated_spans_snapshot = mem::take(&mut *psess.gated_spans.spans.borrow_mut());

        let result = tt_parser.parse_tt(&mut Cow::Borrowed(&body_parser), body, track);
        track.after_arm(true, &result);

        match result {
            mbe_macro_parser::ParseResult::Success(named_matches) => {
                psess.gated_spans.merge(gated_spans_snapshot);
                return Ok((i, rule, named_matches));
            }
            mbe_macro_parser::ParseResult::Failure(_) => {
                mem::swap(&mut gated_spans_snapshot, &mut psess.gated_spans.spans.borrow_mut())
            }
            mbe_macro_parser::ParseResult::Error(_, _) => return Err(CanRetry::Yes),
            mbe_macro_parser::ErrorReported(guar) => return Err(CanRetry::No(guar)),
        }
    }

    Err(CanRetry::Yes)
}

