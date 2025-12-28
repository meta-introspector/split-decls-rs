macro_rules! deps {
    () => {
        SourceName!();
    };
}

macro_rules! AbiTag {
    () => {
        deps!();
        # [doc = " The `<abi-tag>` non-terminal."] # [doc = ""] # [doc = " ```text"] # [doc = " <abi-tag> ::= B <source-name>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct AbiTag (SourceName) ;
    };
}

AbiTag!();