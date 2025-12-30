// Generated macro for impl_1880 (impl)
macro_rules! Depcrate_sink_feedimpl_1880 {
() => {
// Module: crate::sink::feed
// Provides: {"impl_1880"}
// Dependencies: {}
impl < 'a , Si : Sink < Item > + Unpin + ? Sized , Item > Feed < 'a , Si , Item > { pub (super) fn new (sink : & 'a mut Si , item : Item) -> Self { Feed { sink , item : Some (item) } } pub (super) fn sink_pin_mut (& mut self) -> Pin < & mut Si > { Pin :: new (self . sink) } pub (super) fn is_item_pending (& self) -> bool { self . item . is_some () } }
};
}
