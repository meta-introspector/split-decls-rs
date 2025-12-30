// Generated macro for impl_427 (impl)
macro_rules! Depcrate_iter_collect_consumerimpl_427 {
() => {
// Module: crate::iter::collect::consumer
// Provides: {"impl_427"}
// Dependencies: {}
impl < 'c , T > CollectResult < 'c , T > { # [doc = " The current length of the collect result"] pub (super) fn len (& self) -> usize { self . initialized_len } # [doc = " Release ownership of the slice of elements, and return the length"] pub (super) fn release_ownership (mut self) -> usize { let ret = self . initialized_len ; self . initialized_len = 0 ; ret } }
};
}
