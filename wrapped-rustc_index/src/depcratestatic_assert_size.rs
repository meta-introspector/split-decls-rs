// Generated macro for static_assert_size (macro)
macro_rules! Depcratestatic_assert_size {
() => {
// Module: crate
// Provides: {"static_assert_size"}
// Dependencies: {}
# [macro_export] # [cfg (feature = "rustc_randomized_layouts")] macro_rules ! static_assert_size { ($ ty : ty , $ size : expr) => { const _ : (usize , usize) = ($ size , :: std :: mem :: size_of ::<$ ty > ()) ; } ; }
};
}
