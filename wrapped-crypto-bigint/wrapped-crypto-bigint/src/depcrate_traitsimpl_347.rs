// Generated macro for impl_347 (impl)
macro_rules! Depcrate_traitsimpl_347 {
() => {
// Module: crate::traits
// Provides: {"impl_347"}
// Dependencies: {}
# [allow (deprecated)] impl < T , Rhs > InvMod < Rhs > for T where T : InvertMod < Rhs > , { type Output = < T as InvertMod < Rhs > > :: Output ; fn inv_mod (& self , p : & Rhs) -> CtOption < Self :: Output > { self . invert_mod (p) } }
};
}
