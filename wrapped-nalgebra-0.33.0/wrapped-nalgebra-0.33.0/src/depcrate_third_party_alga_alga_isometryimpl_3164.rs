// Generated macro for impl_3164 (impl)
macro_rules! Depcrate_third_party_alga_alga_isometryimpl_3164 {
() => {
// Module: crate::third_party::alga::alga_isometry
// Provides: {"impl_3164"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > Similarity < Point < T , D > > for Isometry < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { type Scaling = Id ; # [inline] fn translation (& self) -> Translation < T , D > { self . translation } # [inline] fn rotation (& self) -> R { self . rotation . clone () } # [inline] fn scaling (& self) -> Id { Id :: new () } }
};
}
