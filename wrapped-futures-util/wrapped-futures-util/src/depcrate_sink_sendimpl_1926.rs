// Generated macro for impl_1926 (impl)
macro_rules! Depcrate_sink_sendimpl_1926 {
() => {
// Module: crate::sink::send
// Provides: {"impl_1926"}
// Dependencies: {}
impl < 'a , Si : Sink < Item > + Unpin + ? Sized , Item > Send < 'a , Si , Item > { pub (super) fn new (sink : & 'a mut Si , item : Item) -> Self { Self { feed : Feed :: new (sink , item) } } }
};
}
