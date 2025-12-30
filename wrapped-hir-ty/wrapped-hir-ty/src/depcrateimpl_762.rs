// Generated macro for impl_762 (impl)
macro_rules! Depcrateimpl_762 {
() => {
// Module: crate
// Provides: {"impl_762"}
// Dependencies: {}
impl ComplexMemoryMap < '_ > { fn insert (& mut self , addr : usize , val : Box < [u8] >) { match self . memory . entry (addr) { Entry :: Occupied (mut e) => { if e . get () . len () < val . len () { e . insert (val) ; } } Entry :: Vacant (e) => { e . insert (val) ; } } } }
};
}
