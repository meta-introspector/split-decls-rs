// Generated macro for impl_834 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_834 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_834"}
// Dependencies: {}
impl < St , T > fmt :: Debug for NextIfEq < '_ , St , T > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , T : ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("NextIfEq") . field ("inner" , & self . inner . inner . as_ref () . map (| (s , _f) | s)) . finish () } }
};
}
