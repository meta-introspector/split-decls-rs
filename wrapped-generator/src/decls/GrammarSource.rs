macro_rules! GrammarSource {
    () => {
        # [derive (Debug , PartialEq)] pub (crate) enum GrammarSource { File (String) , Inline (String) , }
    };
}

GrammarSource!()