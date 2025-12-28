macro_rules! BreakFinder {
    () => {
        # [doc = " Look for `break` expressions within any arbitrary expressions. We'll do this to infer"] # [doc = " whether this is a case where the moved value would affect the exit of a loop, making it"] # [doc = " unsuitable for a `.clone()` suggestion."] struct BreakFinder { found_breaks : Vec < (hir :: Destination , Span) > , found_continues : Vec < (hir :: Destination , Span) > , }
    };
}

BreakFinder!();