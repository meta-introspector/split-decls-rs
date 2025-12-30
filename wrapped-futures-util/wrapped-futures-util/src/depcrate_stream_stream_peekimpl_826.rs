// Generated macro for impl_826 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_826 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_826"}
// Dependencies: {}
impl < St > fmt :: Debug for PeekMut < '_ , St > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PeekMut") . field ("inner" , & self . inner) . finish () } }
};
}
