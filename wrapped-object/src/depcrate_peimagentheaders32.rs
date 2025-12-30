// Generated macro for ImageNtHeaders32 (struct)
macro_rules! Depcrate_peImageNtHeaders32 {
() => {
// Module: crate::pe
// Provides: {"ImageNtHeaders32"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageNtHeaders32 { pub signature : U32 < LE > , pub file_header : ImageFileHeader , pub optional_header : ImageOptionalHeader32 , }
};
}
