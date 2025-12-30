// Generated macro for impl_1413 (impl)
macro_rules! Depcrateimpl_1413 {
() => {
// Module: crate
// Provides: {"impl_1413"}
// Dependencies: {}
impl Direction { # [doc = " Return the opposite `Direction`."] # [inline] pub fn opposite (self) -> Direction { match self { Outgoing => Incoming , Incoming => Outgoing , } } # [doc = " Return `0` for `Outgoing` and `1` for `Incoming`."] # [inline] pub fn index (self) -> usize { (self as usize) & 0x1 } }
};
}
