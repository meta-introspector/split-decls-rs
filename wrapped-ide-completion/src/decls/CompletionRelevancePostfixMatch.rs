macro_rules! CompletionRelevancePostfixMatch {
    () => {
        # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum CompletionRelevancePostfixMatch { # [doc = " Set in cases when item is postfix, but not exact"] NonExact , # [doc = " This is set in cases like these:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " (a > b).not$0"] # [doc = " ```"] # [doc = ""] # [doc = " Basically, we want to guarantee that postfix snippets always takes"] # [doc = " precedence over everything else."] Exact , }
    };
}

CompletionRelevancePostfixMatch!();