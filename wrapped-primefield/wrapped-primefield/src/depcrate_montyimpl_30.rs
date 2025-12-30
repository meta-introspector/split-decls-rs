// Generated macro for impl_30 (impl)
macro_rules! Depcrate_montyimpl_30 {
() => {
// Module: crate::monty
// Provides: {"impl_30"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > Field for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , MontyFieldBytes < MOD , LIMBS > : Copy , Uint < LIMBS > : ArrayEncoding , { const ZERO : Self = Self :: ZERO ; const ONE : Self = Self :: ONE ; fn try_from_rng < R : rand_core :: TryRngCore + ? Sized > (rng : & mut R) -> Result < Self , R :: Error > { let mut bytes = MontyFieldBytes :: < MOD , LIMBS > :: default () ; loop { rng . try_fill_bytes (& mut bytes) ? ; if let Some (fe) = Self :: from_bytes (& bytes) . into () { return Ok (fe) ; } } } fn is_zero (& self) -> Choice { Self :: ZERO . ct_eq (self) } fn square (& self) -> Self { self . square () } fn double (& self) -> Self { self . double () } fn invert (& self) -> CtOption < Self > { self . invert () } fn sqrt (& self) -> CtOption < Self > { self . sqrt () } fn sqrt_ratio (num : & Self , div : & Self) -> (Choice , Self) { ff :: helpers :: sqrt_ratio_generic (num , div) } }
};
}
