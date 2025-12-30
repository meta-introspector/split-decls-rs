// Generated macro for impl_893 (impl)
macro_rules! Depcrate_stream_stream_take_whileimpl_893 {
() => {
// Module: crate::stream::stream::take_while
// Provides: {"impl_893"}
// Dependencies: {}
impl < St , Fut , F > fmt :: Debug for TakeWhile < St , Fut , F > where St : Stream + fmt :: Debug , St :: Item : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TakeWhile") . field ("stream" , & self . stream) . field ("pending_fut" , & self . pending_fut) . field ("pending_item" , & self . pending_item) . field ("done_taking" , & self . done_taking) . finish () } }
};
}
