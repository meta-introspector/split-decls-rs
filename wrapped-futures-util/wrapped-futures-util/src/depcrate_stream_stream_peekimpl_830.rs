// Generated macro for impl_830 (impl)
macro_rules! Depcrate_stream_stream_peekimpl_830 {
() => {
// Module: crate::stream::stream::peek
// Provides: {"impl_830"}
// Dependencies: {}
impl < St , F > fmt :: Debug for NextIf < '_ , St , F > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("NextIf") . field ("inner" , & self . inner . as_ref () . map (| (s , _f) | s)) . finish () } }
};
}
