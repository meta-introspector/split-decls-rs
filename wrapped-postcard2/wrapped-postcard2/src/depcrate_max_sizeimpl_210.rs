// Generated macro for impl_210 (impl)
macro_rules! Depcrate_max_sizeimpl_210 {
() => {
// Module: crate::max_size
// Provides: {"impl_210"}
// Dependencies: {}
impl < T : MaxSize > MaxSize for RangeInclusive < T > { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE * 2 ; }
};
}
