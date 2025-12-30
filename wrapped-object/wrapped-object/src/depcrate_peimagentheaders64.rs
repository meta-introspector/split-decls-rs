// Generated macro for ImageNtHeaders64 (struct)
macro_rules! Depcrate_peImageNtHeaders64 {
() => {
// Module: crate::pe
// Provides: {"ImageNtHeaders64"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageNtHeaders64 { pub signature : U32 < LE > , pub file_header : ImageFileHeader , pub optional_header : ImageOptionalHeader64 , }
};
}
