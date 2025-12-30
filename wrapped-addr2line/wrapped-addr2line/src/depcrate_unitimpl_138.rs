// Generated macro for impl_138 (impl)
macro_rules! Depcrate_unitimpl_138 {
() => {
// Module: crate::unit
// Provides: {"impl_138"}
// Dependencies: {}
impl < 'ctx , R > Iterator for LocationRangeIter < 'ctx , R > where R : gimli :: Reader + 'ctx , { type Item = (u64 , u64 , Location < 'ctx >) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . next_loc () . unwrap_or_default () } }
};
}
