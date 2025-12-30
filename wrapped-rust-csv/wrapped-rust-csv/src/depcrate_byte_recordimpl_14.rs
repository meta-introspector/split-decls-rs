// Generated macro for impl_14 (impl)
macro_rules! Depcrate_byte_recordimpl_14 {
() => {
// Module: crate::byte_record
// Provides: {"impl_14"}
// Dependencies: {}
impl fmt :: Debug for ByteRecord { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "ByteRecord(") ? ; f . debug_list () . entries (self . iter () . map (crate :: debug :: Bytes)) . finish () ? ; write ! (f , ")") ? ; Ok (()) } }
};
}
