// Generated macro for impl_26 (impl)
macro_rules! Depcrate_clockimpl_26 {
() => {
// Module: crate::clock
// Provides: {"impl_26"}
// Dependencies: {}
impl < T : GetTime > Clock < T > { pub fn now () -> Clock < T > { Clock { dur : T :: time () , _marker : PhantomData , } } pub fn elapsed (& self) -> Duration { T :: time () - self . dur } }
};
}
