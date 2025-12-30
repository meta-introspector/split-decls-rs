// Generated macro for impl_3300 (impl)
macro_rules! Depcrate_third_party_alga_alga_translationimpl_3300 {
() => {
// Module: crate::third_party::alga::alga_translation
// Provides: {"impl_3300"}
// Dependencies: {}
# [doc = " Subgroups of the n-dimensional translation group `T(n)`."] impl < T : RealField + simba :: scalar :: RealField , const D : usize > AlgaTranslation < Point < T , D > > for Translation < T , D > { # [inline] fn to_vector (& self) -> SVector < T , D > { self . vector } # [inline] fn from_vector (v : SVector < T , D >) -> Option < Self > { Some (Self :: from (v)) } # [inline] fn powf (& self , n : T) -> Option < Self > { Some (Self :: from (self . vector * n)) } # [inline] fn translation_between (a : & Point < T , D > , b : & Point < T , D >) -> Option < Self > { Some (Self :: from (b - a)) } }
};
}
