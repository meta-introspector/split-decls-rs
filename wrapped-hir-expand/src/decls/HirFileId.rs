macro_rules! deps {
    () => {
        MacroCallId!();
    };
}

macro_rules! HirFileId {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum HirFileId { FileId (EditionedFileId) , MacroFile (MacroCallId) , }
    };
}

HirFileId!()