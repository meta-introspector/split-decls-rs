// Generated macro for impl_1011 (impl)
macro_rules! Depcrate_stream_stream_scanimpl_1011 {
() => {
// Module: crate::stream::stream::scan
// Provides: {"impl_1011"}
// Dependencies: {}
impl < St , S , Fut , F > fmt :: Debug for Scan < St , S , Fut , F > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , S : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Scan") . field ("stream" , & self . stream) . field ("state" , & self . state) . field ("done_taking" , & self . is_done_taking ()) . finish () } }
};
}
