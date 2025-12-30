// Generated macro for impl_676 (impl)
macro_rules! Depcrate_stream_stream_foldimpl_676 {
() => {
// Module: crate::stream::stream::fold
// Provides: {"impl_676"}
// Dependencies: {}
impl < St , Fut , T , F > fmt :: Debug for Fold < St , Fut , T , F > where St : fmt :: Debug , Fut : fmt :: Debug , T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Fold") . field ("stream" , & self . stream) . field ("accum" , & self . accum) . field ("future" , & self . future) . finish () } }
};
}
