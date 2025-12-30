// Generated macro for impl_154 (impl)
macro_rules! Depcrate_filesimpl_154 {
() => {
// Module: crate::files
// Provides: {"impl_154"}
// Dependencies: {}
impl < T > InFile < T > { pub fn into_real_file (self) -> Result < InRealFile < T > , InFile < T > > { match self . file_id { HirFileId :: FileId (file_id) => Ok (InRealFile { file_id , value : self . value }) , HirFileId :: MacroFile (_) => Err (self) , } } }
};
}
