macro_rules! delim_to_external {
    () => {
        fn delim_to_external < S > (d : tt :: Delimiter < S >) -> proc_macro :: Delimiter { match d . kind { tt :: DelimiterKind :: Parenthesis => proc_macro :: Delimiter :: Parenthesis , tt :: DelimiterKind :: Brace => proc_macro :: Delimiter :: Brace , tt :: DelimiterKind :: Bracket => proc_macro :: Delimiter :: Bracket , tt :: DelimiterKind :: Invisible => proc_macro :: Delimiter :: None , } }
    };
}

delim_to_external!();