// Generated macro for impl_3297 (impl)
macro_rules! Depcrate_third_party_alga_alga_translationimpl_3297 {
() => {
// Module: crate::third_party::alga::alga_translation
// Provides: {"impl_3297"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , const D : usize > Similarity < Point < T , D > > for Translation < T , D > { type Scaling = Id ; # [inline] fn translation (& self) -> Self { * self } # [inline] fn rotation (& self) -> Id { Id :: new () } # [inline] fn scaling (& self) -> Id { Id :: new () } }
};
}
