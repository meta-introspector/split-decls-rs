// Generated macro for impl_928 (impl)
macro_rules! Depcrateimpl_928 {
() => {
// Module: crate
// Provides: {"impl_928"}
// Dependencies: {}
impl ComplexMemoryMap { fn insert (& mut self , addr : usize , val : Box < [u8] >) { match self . memory . entry (addr) { Entry :: Occupied (mut e) => { if e . get () . len () < val . len () { e . insert (val) ; } } Entry :: Vacant (e) => { e . insert (val) ; } } } }
};
}
