macro_rules! delim_to_internal {
    () => {
        fn delim_to_internal < S > (d : proc_macro :: Delimiter , span : bridge :: DelimSpan < S >) -> tt :: Delimiter < S > { let kind = match d { proc_macro :: Delimiter :: Parenthesis => tt :: DelimiterKind :: Parenthesis , proc_macro :: Delimiter :: Brace => tt :: DelimiterKind :: Brace , proc_macro :: Delimiter :: Bracket => tt :: DelimiterKind :: Bracket , proc_macro :: Delimiter :: None => tt :: DelimiterKind :: Invisible , } ; tt :: Delimiter { open : span . open , close : span . close , kind } }
    };
}

delim_to_internal!();