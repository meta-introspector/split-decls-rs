// Generated macro for impl_936 (impl)
macro_rules! Depcrate_read_pe_fileimpl_936 {
() => {
// Module: crate::read::pe::file
// Provides: {"impl_936"}
// Dependencies: {}
impl ImageNtHeaders for pe :: ImageNtHeaders64 { type ImageOptionalHeader = pe :: ImageOptionalHeader64 ; type ImageThunkData = pe :: ImageThunkData64 ; # [inline] fn is_type_64 (& self) -> bool { true } # [inline] fn is_valid_optional_magic (& self) -> bool { self . optional_header . magic . get (LE) == pe :: IMAGE_NT_OPTIONAL_HDR64_MAGIC } # [inline] fn signature (& self) -> u32 { self . signature . get (LE) } # [inline] fn file_header (& self) -> & pe :: ImageFileHeader { & self . file_header } # [inline] fn optional_header (& self) -> & Self :: ImageOptionalHeader { & self . optional_header } }
};
}
