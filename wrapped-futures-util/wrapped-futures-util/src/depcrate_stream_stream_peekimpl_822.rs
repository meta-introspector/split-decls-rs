// Generated macro for impl_822 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_822 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_822"}
// Dependencies: {}
impl < St > fmt :: Debug for Peek < '_ , St > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Peek") . field ("inner" , & self . inner) . finish () } }
};
}
