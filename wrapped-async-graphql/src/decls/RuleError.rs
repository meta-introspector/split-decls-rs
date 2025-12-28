macro_rules! RuleError {
    () => {
        # [derive (Debug , PartialEq)] pub (crate) struct RuleError { pub (crate) locations : Vec < Pos > , pub (crate) message : String , }
    };
}

RuleError!();