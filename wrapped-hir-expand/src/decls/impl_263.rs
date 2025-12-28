macro_rules! deps {
    () => {
        HirFileId!();
        MacroCallId!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl HirFileId { # [inline] pub fn macro_file (self) -> Option < MacroCallId > { match self { HirFileId :: FileId (_) => None , HirFileId :: MacroFile (it) => Some (it) , } } # [inline] pub fn is_macro (self) -> bool { matches ! (self , HirFileId :: MacroFile (_)) } # [inline] pub fn file_id (self) -> Option < EditionedFileId > { match self { HirFileId :: FileId (it) => Some (it) , HirFileId :: MacroFile (_) => None , } } }
    };
}

impl_263!()