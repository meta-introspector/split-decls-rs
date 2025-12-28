macro_rules! ParseNode {
    () => {
        # [doc = " A list of parsers that parsing can fail on. This is used for pretty-printing errors"] # [derive (PartialEq , Debug , Clone , Copy)] pub (crate) enum ParseNode { SectionHeader , Name , Value , }
    };
}

ParseNode!();