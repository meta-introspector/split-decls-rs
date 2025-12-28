macro_rules! TypeAlias {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct TypeAlias { pub (crate) id : TypeAliasId , }
    };
}

TypeAlias!()