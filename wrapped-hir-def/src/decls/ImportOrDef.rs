macro_rules! deps {
    () => {
        GlobId!();
        ModuleDefId!();
        ImportId!();
        ExternCrate!();
    };
}

macro_rules! ImportOrDef {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum ImportOrDef { Import (ImportId) , Glob (GlobId) , ExternCrate (ExternCrateId) , Def (ModuleDefId) , }
    };
}

ImportOrDef!();