// Generated macro for impl_3314 (impl)
macro_rules! Depcrate_third_party_alga_alga_unit_compleximpl_3314 {
() => {
// Module: crate::third_party::alga::alga_unit_complex
// Provides: {"impl_3314"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > Similarity < Point2 < T > > for UnitComplex < T > { type Scaling = Id ; # [inline] fn translation (& self) -> Id { Id :: new () } # [inline] fn rotation (& self) -> Self { * self } # [inline] fn scaling (& self) -> Id { Id :: new () } }
};
}
