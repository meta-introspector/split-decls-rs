// Generated macro for impl_1940 (impl)
macro_rules! Depcrate_sink_send_allimpl_1940 {
() => {
// Module: crate::sink::send_all
// Provides: {"impl_1940"}
// Dependencies: {}
impl < Si , St > fmt :: Debug for SendAll < '_ , Si , St > where Si : fmt :: Debug + ? Sized , St : fmt :: Debug + TryStream , St :: Ok : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SendAll") . field ("sink" , & self . sink) . field ("stream" , & self . stream) . field ("buffered" , & self . buffered) . finish () } }
};
}
