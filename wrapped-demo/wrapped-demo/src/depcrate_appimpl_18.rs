// Generated macro for impl_18 (impl)
macro_rules! Depcrate_appimpl_18 {
() => {
// Module: crate::app
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a > TabsState < 'a > { pub const fn new (titles : Vec < & 'a str >) -> Self { Self { titles , index : 0 } } pub fn next (& mut self) { self . index = (self . index + 1) % self . titles . len () ; } pub fn previous (& mut self) { if self . index > 0 { self . index -= 1 ; } else { self . index = self . titles . len () - 1 ; } } }
};
}
