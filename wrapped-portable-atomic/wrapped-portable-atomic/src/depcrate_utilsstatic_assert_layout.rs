// Generated macro for static_assert_layout (macro)
macro_rules! Depcrate_utilsstatic_assert_layout {
() => {
// Module: crate::utils
// Provides: {"static_assert_layout"}
// Dependencies: {}
macro_rules ! static_assert_layout { ($ atomic_type : ty , $ value_type : ty) => { static_assert ! (core :: mem :: align_of ::<$ atomic_type > () == core :: mem :: size_of ::<$ atomic_type > ()) ; static_assert ! (core :: mem :: size_of ::<$ atomic_type > () == core :: mem :: size_of ::<$ value_type > ()) ; } ; }
};
}
