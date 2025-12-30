// Generated macro for impl_31 (impl)
macro_rules! Depcrate_montyimpl_31 {
() => {
// Module: crate::monty
// Provides: {"impl_31"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > PrimeField for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , MontyFieldBytes < MOD , LIMBS > : Copy , Uint < LIMBS > : ArrayEncoding , { type Repr = MontyFieldBytes < MOD , LIMBS > ; const MODULUS : & 'static str = MOD :: MODULUS_HEX ; const NUM_BITS : u32 = MOD :: PARAMS . modulus () . as_ref () . bits () ; const CAPACITY : u32 = Self :: NUM_BITS - 1 ; const TWO_INV : Self = Self :: from_u64 (2) . const_invert () ; const MULTIPLICATIVE_GENERATOR : Self = Self :: from_u64 (MOD :: MULTIPLICATIVE_GENERATOR) ; const S : u32 = compute_s (MOD :: PARAMS . modulus () . as_ref ()) ; const ROOT_OF_UNITY : Self = match MOD :: ROOT_OF_UNITY { Some (root_of_unity) => Self :: from_uint_reduced (& root_of_unity) , None => Self :: MULTIPLICATIVE_GENERATOR . pow_vartime (& MOD :: T) , } ; const ROOT_OF_UNITY_INV : Self = Self :: ROOT_OF_UNITY . const_invert () ; const DELTA : Self = Self :: MULTIPLICATIVE_GENERATOR . sqn_vartime (Self :: S as usize) ; fn from_repr (bytes : Self :: Repr) -> CtOption < Self > { Self :: from_bytes (& bytes) } fn to_repr (& self) -> Self :: Repr { self . to_bytes () } fn is_odd (& self) -> Choice { self . is_odd () } }
};
}
