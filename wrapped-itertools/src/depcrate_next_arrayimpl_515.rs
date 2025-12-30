// Generated macro for impl_515 (impl)
macro_rules! Depcrate_next_arrayimpl_515 {
() => {
// Module: crate::next_array
// Provides: {"impl_515"}
// Dependencies: {}
impl < T , const N : usize > Drop for ArrayBuilder < T , N > { fn drop (& mut self) { unsafe { core :: ptr :: drop_in_place (self . as_mut ()) } } }
};
}
