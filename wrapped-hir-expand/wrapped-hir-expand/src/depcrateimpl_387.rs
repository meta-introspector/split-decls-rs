// Generated macro for impl_387 (impl)
macro_rules! Depcrateimpl_387 {
() => {
// Module: crate
// Provides: {"impl_387"}
// Dependencies: {}
impl HirFileId { # [inline] pub fn macro_file (self) -> Option < MacroCallId > { match self { HirFileId :: FileId (_) => None , HirFileId :: MacroFile (it) => Some (it) , } } # [inline] pub fn is_macro (self) -> bool { matches ! (self , HirFileId :: MacroFile (_)) } # [inline] pub fn file_id (self) -> Option < EditionedFileId > { match self { HirFileId :: FileId (it) => Some (it) , HirFileId :: MacroFile (_) => None , } } }
};
}
