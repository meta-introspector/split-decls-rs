// Generated macro for impl_3266 (impl)
macro_rules! Depcrate_third_party_alga_alga_similarityimpl_3266 {
() => {
// Module: crate::third_party::alga::alga_similarity
// Provides: {"impl_3266"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , R , const D : usize > AlgaSimilarity < Point < T , D > > for Similarity < T , R , D > where R : Rotation < Point < T , D > > + AbstractRotation < T , D > , { type Scaling = T ; # [inline] fn translation (& self) -> Translation < T , D > { self . isometry . translation () } # [inline] fn rotation (& self) -> R { self . isometry . rotation () } # [inline] fn scaling (& self) -> T { self . scaling () } }
};
}
