// Generated macro for impl_3139 (impl)
macro_rules! Depcrate_third_party_alga_alga_dual_quaternionimpl_3139 {
() => {
// Module: crate::third_party::alga::alga_dual_quaternion
// Provides: {"impl_3139"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > NormedSpace for DualQuaternion < T > { type RealField = T ; type ComplexField = T ; # [inline] fn norm_squared (& self) -> T { self . real . norm_squared () } # [inline] fn norm (& self) -> T { self . real . norm () } # [inline] fn normalize (& self) -> Self { self . normalize () } # [inline] fn normalize_mut (& mut self) -> T { self . normalize_mut () } # [inline] fn try_normalize (& self , min_norm : T) -> Option < Self > { let real_norm = self . real . norm () ; if real_norm > min_norm { Some (Self :: from_real_and_dual (self . real / real_norm , self . dual / real_norm ,)) } else { None } } # [inline] fn try_normalize_mut (& mut self , min_norm : T) -> Option < T > { let real_norm = self . real . norm () ; if real_norm > min_norm { self . real /= real_norm ; self . dual /= real_norm ; Some (real_norm) } else { None } } }
};
}
