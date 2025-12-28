macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! SourceName {
    () => {
        deps!();
        # [doc = " The `<source-name>` non-terminal."] # [doc = ""] # [doc = " ```text"] # [doc = " <source-name> ::= <positive length number> <identifier>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct SourceName (Identifier) ;
    };
}

SourceName!()