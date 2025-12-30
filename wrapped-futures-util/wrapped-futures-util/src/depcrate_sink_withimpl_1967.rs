// Generated macro for impl_1967 (impl)
macro_rules! Depcrate_sink_withimpl_1967 {
() => {
// Module: crate::sink::with
// Provides: {"impl_1967"}
// Dependencies: {}
impl < Si , Item , U , Fut , F > fmt :: Debug for With < Si , Item , U , Fut , F > where Si : fmt :: Debug , Fut : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("With") . field ("sink" , & self . sink) . field ("state" , & self . state) . finish () } }
};
}
