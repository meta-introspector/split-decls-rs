macro_rules! requires_semi {
    () => {
        pub (crate) fn requires_semi (delimiter : & MacroDelimiter) -> bool { match delimiter { MacroDelimiter :: Paren (_) | MacroDelimiter :: Bracket (_) => true , MacroDelimiter :: Brace (_) => false , } }
    };
}

requires_semi!()