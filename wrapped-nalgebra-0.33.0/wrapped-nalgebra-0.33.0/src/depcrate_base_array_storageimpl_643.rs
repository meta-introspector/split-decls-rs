// Generated macro for impl_643 (impl)
macro_rules! Depcrate_base_array_storageimpl_643 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_643"}
// Dependencies: {}
impl < T : Default , const R : usize , const C : usize > Default for ArrayStorage < T , R , C > where [[T ; R] ; C] : Default , { # [inline] fn default () -> Self { Self (Default :: default ()) } }
};
}
