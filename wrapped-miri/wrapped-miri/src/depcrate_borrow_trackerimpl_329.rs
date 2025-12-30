// Generated macro for impl_329 (impl)
macro_rules! Depcrate_borrow_trackerimpl_329 {
() => {
// Module: crate::borrow_tracker
// Provides: {"impl_329"}
// Dependencies: {}
impl BorTag { pub fn new (i : u64) -> Option < Self > { NonZero :: new (i) . map (BorTag) } pub fn get (& self) -> u64 { self . 0 . get () } pub fn inner (& self) -> NonZero < u64 > { self . 0 } pub fn succ (self) -> Option < Self > { self . 0 . checked_add (1) . map (Self) } # [doc = " The minimum representable tag"] pub fn one () -> Self { Self :: new (1) . unwrap () } }
};
}
