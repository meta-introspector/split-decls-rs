macro_rules! ParsingToken {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub enum ParsingToken { Sensitive { token : String } , Insensitive { token : String } , Range { start : char , end : char } , BuiltInRule , }
    };
}

ParsingToken!()