// Generated macro for NAME_FROM_RESOURCE_ENTRY (function)
macro_rules! Depcrate_ntldrNAME_FROM_RESOURCE_ENTRY {
() => {
// Module: crate::ntldr
// Provides: {"NAME_FROM_RESOURCE_ENTRY"}
// Dependencies: {}
# [inline] pub unsafe fn NAME_FROM_RESOURCE_ENTRY (RootDirectory : PIMAGE_RESOURCE_DIRECTORY , Entry : & IMAGE_RESOURCE_DIRECTORY_ENTRY ,) -> usize { if Entry . u . s () . NameIsString () != 0 { return RootDirectory as usize + Entry . u . s () . NameOffset () as usize ; } * Entry . u . Id () as usize }
};
}
