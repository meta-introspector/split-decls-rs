// Generated macro for impl_3223 (impl)
macro_rules! Depcrate_third_party_alga_alga_quaternionimpl_3223 {
() => {
// Module: crate::third_party::alga::alga_quaternion
// Provides: {"impl_3223"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > NormedSpace for Quaternion < T > { type RealField = T ; type ComplexField = T ; # [inline] fn norm_squared (& self) -> T { self . coords . norm_squared () } # [inline] fn norm (& self) -> T { self . as_vector () . norm () } # [inline] fn normalize (& self) -> Self { let v = self . coords . normalize () ; Self :: from (v) } # [inline] fn normalize_mut (& mut self) -> T { self . coords . normalize_mut () } # [inline] fn try_normalize (& self , min_norm : T) -> Option < Self > { self . coords . try_normalize (min_norm) . map (Self :: from) } # [inline] fn try_normalize_mut (& mut self , min_norm : T) -> Option < T > { self . coords . try_normalize_mut (min_norm) } }
};
}
