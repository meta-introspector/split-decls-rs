// Generated macro for CXIdxIncludedFileInfo (struct)
macro_rules! DepcrateCXIdxIncludedFileInfo {
() => {
// Module: crate
// Provides: {"CXIdxIncludedFileInfo"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXIdxIncludedFileInfo { pub hashLoc : CXIdxLoc , pub filename : * const c_char , pub file : CXFile , pub isImport : c_int , pub isAngled : c_int , pub isModuleImport : c_int , }
};
}
