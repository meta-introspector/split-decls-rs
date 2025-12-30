// Generated macro for tests (module)
macro_rules! Depcrate_montytests {
() => {
// Module: crate::monty
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: MontyFieldElement ; use crate :: { ByteOrder , monty_field_params , test_primefield } ; use bigint :: U256 ; monty_field_params ! (name : FieldParams , modulus : "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff" , uint : U256 , byte_order : ByteOrder :: BigEndian , multiplicative_generator : 6 , doc : "P-256 field modulus") ; # [doc = " P-256 field element"] type FieldElement = MontyFieldElement < FieldParams , { U256 :: LIMBS } > ; test_primefield ! (FieldElement , U256) ; # [test] fn modulus_bits_constant () { assert_eq ! (FieldElement :: NUM_BITS , 256) ; } # [test] fn s_constant () { assert_eq ! (FieldElement :: S , 1) ; } # [test] fn computed_delta_constant () { assert_eq ! (FieldElement :: DELTA , FieldElement :: from_u64 (36)) ; } }
};
}
