// Generated macro for impl_253 (impl)
macro_rules! Depcrate_frameimpl_253 {
() => {
// Module: crate::frame
// Provides: {"impl_253"}
// Dependencies: {}
impl Debug for CloseTriggerFrame { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let repr = match & self . comparator { Comparator :: Frame (frame) => format ! ("{frame:?}") , Comparator :: Fn (_) => "closure" . to_string () , } ; write ! (f , "CloseTriggerFrame {{ stream_id: {}, comparator: {repr} }}" , self . stream_id) } }
};
}
