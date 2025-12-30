// Generated macro for impl_58 (impl)
macro_rules! Depcrate_foreign_core_arrayimpl_58 {
() => {
// Module: crate::foreign::core::array
// Provides: {"impl_58"}
// Dependencies: {}
impl < T , const N : usize > Drop for ArrayGuard < T , N > { fn drop (& mut self) { debug_assert ! (self . initialized <= N) ; let initialized_part = ptr :: slice_from_raw_parts_mut (self . dst , self . initialized) ; unsafe { ptr :: drop_in_place (initialized_part) ; } } }
};
}
