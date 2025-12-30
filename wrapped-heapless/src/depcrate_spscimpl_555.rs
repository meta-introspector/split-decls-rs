// Generated macro for impl_555 (impl)
macro_rules! Depcrate_spscimpl_555 {
() => {
// Module: crate::spsc
// Provides: {"impl_555"}
// Dependencies: {}
impl < T , S : Storage > Drop for QueueInner < T , S > { fn drop (& mut self) { for item in self { unsafe { ptr :: drop_in_place (item) ; } } } }
};
}
