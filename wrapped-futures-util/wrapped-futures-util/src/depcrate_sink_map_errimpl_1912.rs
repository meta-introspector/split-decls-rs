// Generated macro for impl_1912 (impl)
macro_rules! Depcrate_sink_map_errimpl_1912 {
() => {
// Module: crate::sink::map_err
// Provides: {"impl_1912"}
// Dependencies: {}
impl < Si , F > SinkMapErr < Si , F > { pub (super) fn new (sink : Si , f : F) -> Self { Self { sink , f : Some (f) } } delegate_access_inner ! (sink , Si , ()) ; fn take_f (self : Pin < & mut Self >) -> F { self . project () . f . take () . expect ("polled MapErr after completion") } }
};
}
