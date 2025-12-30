// Generated macro for impl_48 (impl)
macro_rules! Depcrate_montyimpl_48 {
() => {
// Module: crate::monty
// Provides: {"impl_48"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > Invert for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , MontyForm < MOD , LIMBS > : Invert < Output = CtOption < MontyForm < MOD , LIMBS > > > , { type Output = CtOption < Self > ; fn invert (& self) -> CtOption < Self > { Self :: invert (self) } }
};
}
