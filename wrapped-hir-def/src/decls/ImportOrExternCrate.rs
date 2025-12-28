macro_rules! deps {
    () => {
        ExternCrate!();
        ImportId!();
        GlobId!();
    };
}

macro_rules! ImportOrExternCrate {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum ImportOrExternCrate { Glob (GlobId) , Import (ImportId) , ExternCrate (ExternCrateId) , }
    };
}

ImportOrExternCrate!();