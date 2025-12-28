macro_rules! deps {
    () => {
        ModuleDefId!();
        ImportId!();
        ExternCrate!();
        GlobId!();
    };
}

macro_rules! ImportOrDef {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum ImportOrDef { Import (ImportId) , Glob (GlobId) , ExternCrate (ExternCrateId) , Def (ModuleDefId) , }
    };
}

ImportOrDef!()