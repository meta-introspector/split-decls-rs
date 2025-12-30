// Generated macro for impl_92 (impl)
macro_rules! Depcrate_easy_handleimpl_92 {
() => {
// Module: crate::easy::handle
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'easy , 'data > Drop for Transfer < 'easy , 'data > { fn drop (& mut self) { assert ! (self . easy . inner . get_ref () . borrowed . get () . is_null ()) ; } }
};
}
