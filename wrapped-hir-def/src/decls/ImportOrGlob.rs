macro_rules! deps {
    () => {
        GlobId!();
        ImportId!();
    };
}

macro_rules! ImportOrGlob {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum ImportOrGlob { Glob (GlobId) , Import (ImportId) , }
    };
}

ImportOrGlob!()