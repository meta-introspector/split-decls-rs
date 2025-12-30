// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
# [cfg (feature = "bytes")] impl Buffer for BytesMut { fn len (& self) -> usize { BytesMut :: len (self) } fn is_empty (& self) -> bool { BytesMut :: is_empty (self) } fn extend_from_slice (& mut self , other : & [u8]) -> Result < () > { BytesMut :: extend_from_slice (self , other) ; Ok (()) } fn truncate (& mut self , len : usize) { BytesMut :: truncate (self , len) ; } }
};
}
