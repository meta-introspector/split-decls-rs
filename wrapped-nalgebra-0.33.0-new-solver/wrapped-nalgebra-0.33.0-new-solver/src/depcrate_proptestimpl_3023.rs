// Generated macro for impl_3023 (impl)
macro_rules! Depcrate_proptestimpl_3023 {
() => {
// Module: crate::proptest
// Provides: {"impl_3023"}
// Dependencies: {}
impl < D : Dim > DimRange < D > { # [doc = " The lower bound for dimensions generated."] pub fn lower_bound (& self) -> D { * self . 0 . start () } # [doc = " The upper bound for dimensions generated."] pub fn upper_bound (& self) -> D { * self . 0 . end () } }
};
}
