// Generated macro for impl_22 (impl)
macro_rules! Depcrate_appimpl_22 {
() => {
// Module: crate::app
// Provides: {"impl_22"}
// Dependencies: {}
impl < S > Signal < S > where S : Iterator , { fn on_tick (& mut self) { self . points . drain (0 .. self . tick_rate) ; self . points . extend (self . source . by_ref () . take (self . tick_rate)) ; } }
};
}
