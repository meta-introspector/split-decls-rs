// Generated macro for monty_field_params_with_root_of_unity (macro)
macro_rules! Depcrate_macrosmonty_field_params_with_root_of_unity {
() => {
// Module: crate::macros
// Provides: {"monty_field_params_with_root_of_unity"}
// Dependencies: {}
# [doc = " Same as [`monty_field_params!`], but with a precomputed `ROOT_OF_UNITY` constant."] # [macro_export] macro_rules ! monty_field_params_with_root_of_unity { (name : $ name : ident , modulus : $ modulus_hex : expr , uint : $ uint : ty , byte_order : $ byte_order : expr , multiplicative_generator : $ multiplicative_generator : expr , root_of_unity : $ root_of_unity : expr , doc : $ doc : expr) => { use $ crate :: bigint :: modular :: ConstMontyParams ; $ crate :: bigint :: const_monty_params ! ($ name , $ uint , $ modulus_hex , $ doc) ; impl $ crate :: MontyFieldParams < { <$ uint >:: LIMBS } > for $ name { type ByteSize = $ crate :: bigint :: hybrid_array :: typenum :: U < { $ name :: PARAMS . modulus () . as_ref () . bits () . div_ceil (8) as usize } , >; const BYTE_ORDER : $ crate :: ByteOrder = $ byte_order ; const MODULUS_HEX : &'static str = $ modulus_hex ; const MULTIPLICATIVE_GENERATOR : u64 = $ multiplicative_generator ; const T : $ uint = $ crate :: compute_t ($ name :: PARAMS . modulus () . as_ref ()) ; const ROOT_OF_UNITY : Option <$ uint > = $ root_of_unity ; } } ; }
};
}
