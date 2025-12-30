// Generated macro for alloc_support (module)
macro_rules! Depcrate_seqalloc_support {
() => {
// Module: crate::seq
// Provides: {"alloc_support"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use crate :: std :: vec :: Vec ; impl < 'sval , T : ValueRef < 'sval > > ValueRef < 'sval > for Vec < T > { fn stream_ref < S : Stream < 'sval > + ? Sized > (& self , stream : & mut S) -> Result { (& * * self) . stream_ref (stream) } } }
};
}
