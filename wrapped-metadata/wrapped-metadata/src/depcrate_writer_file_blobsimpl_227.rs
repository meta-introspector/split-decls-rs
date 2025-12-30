// Generated macro for impl_227 (impl)
macro_rules! Depcrate_writer_file_blobsimpl_227 {
() => {
// Module: crate::writer::file::blobs
// Provides: {"impl_227"}
// Dependencies: {}
impl Blobs { pub fn insert (& mut self , value : & [u8]) -> id :: BlobId { if value . is_empty () { return id :: BlobId (0) ; } if let Some (pos) = self . map . get (value) { return * pos ; } let pos = id :: BlobId (self . stream . len () . try_into () . unwrap ()) ; self . map . insert (value . to_vec () , pos) ; let len = value . len () ; match len { 0 ..= 0x7F => self . stream . push (len as u8) , 0x80 ..= 0x3FFF => { self . stream . push ((0x80 | (len >> 8)) as u8) ; self . stream . push ((0xFF & len) as u8) ; } _ => { self . stream . push ((0xC0 | (len >> 24)) as u8) ; self . stream . push ((0xFF & (len >> 16)) as u8) ; self . stream . push ((0xFF & (len >> 8)) as u8) ; self . stream . push ((0xFF & len) as u8) ; } } self . stream . extend_from_slice (value) ; pos } pub fn into_stream (self) -> Vec < u8 > { self . stream . into_stream () } }
};
}
