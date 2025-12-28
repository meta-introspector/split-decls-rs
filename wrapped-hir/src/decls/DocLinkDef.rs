macro_rules! deps {
    () => {
        Trait!();
        ModuleDef!();
        Field!();
    };
}

macro_rules! DocLinkDef {
    () => {
        deps!();
        # [doc = " Subset of `ide_db::Definition` that doc links can resolve to."] pub enum DocLinkDef { ModuleDef (ModuleDef) , Field (Field) , SelfType (Trait) , }
    };
}

DocLinkDef!();