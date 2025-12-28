macro_rules! deps {
    () => {
        SimpleId!();
    };
}

macro_rules! UnresolvedQualifierLevel {
    () => {
        deps!();
        # [doc = " The `<unresolved-qualifier-level>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <unresolved-qualifier-level> ::= <simple-id>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct UnresolvedQualifierLevel (SimpleId) ;
    };
}

UnresolvedQualifierLevel!()