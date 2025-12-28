macro_rules! deps {
    () => {
        Field!();
        ModuleDef!();
        Trait!();
    };
}

macro_rules! DocLinkDef {
    () => {
        deps!();
        # [doc = " Subset of `ide_db::Definition` that doc links can resolve to."] pub enum DocLinkDef { ModuleDef (ModuleDef) , Field (Field) , SelfType (Trait) , }
    };
}

DocLinkDef!()