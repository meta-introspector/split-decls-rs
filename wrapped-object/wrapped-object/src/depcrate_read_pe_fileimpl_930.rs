// Generated macro for impl_930 (impl)
macro_rules! Depcrate_read_pe_fileimpl_930 {
() => {
// Module: crate::read::pe::file
// Provides: {"impl_930"}
// Dependencies: {}
impl pe :: ImageDosHeader { # [doc = " Read the DOS header."] # [doc = ""] # [doc = " Also checks that the `e_magic` field in the header is valid."] pub fn parse < 'data , R : ReadRef < 'data > > (data : R) -> read :: Result < & 'data Self > { let dos_header = data . read_at :: < pe :: ImageDosHeader > (0) . read_error ("Invalid DOS header size or alignment") ? ; if dos_header . e_magic . get (LE) != pe :: IMAGE_DOS_SIGNATURE { return Err (Error ("Invalid DOS magic")) ; } Ok (dos_header) } # [doc = " Return the file offset of the nt_headers."] # [inline] pub fn nt_headers_offset (& self) -> u32 { self . e_lfanew . get (LE) } }
};
}
