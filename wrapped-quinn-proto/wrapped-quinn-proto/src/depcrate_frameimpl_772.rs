// Generated macro for impl_772 (impl)
macro_rules! Depcrate_frameimpl_772 {
() => {
// Module: crate::frame
// Provides: {"impl_772"}
// Dependencies: {}
impl Datagram { pub (crate) fn encode (& self , length : bool , out : & mut Vec < u8 >) { out . write (FrameType (* DATAGRAM_TYS . start () | u64 :: from (length))) ; if length { out . write (VarInt :: from_u64 (self . data . len () as u64) . unwrap ()) ; } out . extend_from_slice (& self . data) ; } pub (crate) fn size (& self , length : bool) -> usize { 1 + if length { VarInt :: from_u64 (self . data . len () as u64) . unwrap () . size () } else { 0 } + self . data . len () } }
};
}
