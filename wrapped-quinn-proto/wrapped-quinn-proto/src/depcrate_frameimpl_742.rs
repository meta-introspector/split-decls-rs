// Generated macro for impl_742 (impl)
macro_rules! Depcrate_frameimpl_742 {
() => {
// Module: crate::frame
// Provides: {"impl_742"}
// Dependencies: {}
impl StreamMeta { pub (crate) fn encode < W : BufMut > (& self , length : bool , out : & mut W) { let mut ty = * STREAM_TYS . start () ; if self . offsets . start != 0 { ty |= 0x04 ; } if length { ty |= 0x02 ; } if self . fin { ty |= 0x01 ; } out . write_var (ty) ; out . write (self . id) ; if self . offsets . start != 0 { out . write_var (self . offsets . start) ; } if length { out . write_var (self . offsets . end - self . offsets . start) ; } } }
};
}
