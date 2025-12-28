macro_rules! deps {
    () => {
        TypeAlias!();
        AssocItemLoc!();
    };
}

macro_rules! TypeAliasLoc {
    () => {
        deps!();
        type TypeAliasLoc = AssocItemLoc < ast :: TypeAlias > ;
    };
}

TypeAliasLoc!()