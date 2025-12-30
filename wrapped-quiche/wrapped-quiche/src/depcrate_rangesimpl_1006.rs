// Generated macro for impl_1006 (impl)
macro_rules! Depcrate_rangesimpl_1006 {
() => {
// Module: crate::ranges
// Provides: {"impl_1006"}
// Dependencies: {}
impl PartialEq < Range < u64 > > for RangeSet { fn eq (& self , other : & Range < u64 >) -> bool { if self . len () != 1 { return false ; } let range = self . iter () . next () . unwrap () ; range == * other } }
};
}
