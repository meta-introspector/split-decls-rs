// Generated macro for impl_1969 (impl)
macro_rules! Depcrate_sink_withimpl_1969 {
() => {
// Module: crate::sink::with
// Provides: {"impl_1969"}
// Dependencies: {}
impl < Si , Item , U , Fut , F > Clone for With < Si , Item , U , Fut , F > where Si : Clone , F : Clone , Fut : Clone , { fn clone (& self) -> Self { Self { state : self . state . clone () , sink : self . sink . clone () , f : self . f . clone () , _phantom : PhantomData , } } }
};
}
