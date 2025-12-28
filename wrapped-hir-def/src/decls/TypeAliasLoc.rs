macro_rules! deps {
    () => {
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