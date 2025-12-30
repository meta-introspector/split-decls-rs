// Generated macro for impl_1968 (impl)
macro_rules! Depcrate_sink_withimpl_1968 {
() => {
// Module: crate::sink::with
// Provides: {"impl_1968"}
// Dependencies: {}
impl < Si , Item , U , Fut , F > With < Si , Item , U , Fut , F > where Si : Sink < Item > , F : FnMut (U) -> Fut , Fut : Future , { pub (super) fn new < E > (sink : Si , f : F) -> Self where Fut : Future < Output = Result < Item , E > > , E : From < Si :: Error > , { Self { state : None , sink , f , _phantom : PhantomData } } }
};
}
