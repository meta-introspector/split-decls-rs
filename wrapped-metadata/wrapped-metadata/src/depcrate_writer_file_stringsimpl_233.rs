// Generated macro for impl_233 (impl)
macro_rules! Depcrate_writer_file_stringsimpl_233 {
() => {
// Module: crate::writer::file::strings
// Provides: {"impl_233"}
// Dependencies: {}
impl Strings { pub fn insert (& mut self , value : & str) -> id :: StringId { if value . is_empty () { return id :: StringId (0) ; } if let Some (pos) = self . map . get (value) { return * pos ; } let pos = id :: StringId (self . stream . len () . try_into () . unwrap ()) ; self . map . insert (value . to_string () , pos) ; self . stream . extend_from_slice (value . as_bytes ()) ; self . stream . push (0) ; pos } pub fn into_stream (self) -> Vec < u8 > { self . stream . into_stream () } }
};
}
