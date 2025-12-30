// Generated macro for monty_field_params (macro)
macro_rules! Depcrate_macrosmonty_field_params {
() => {
// Module: crate::macros
// Provides: {"monty_field_params"}
// Dependencies: {}
# [doc = " Creates a ZST representing the Montgomery parameters for a given field modulus."] # [doc = ""] # [doc = " Accepts the following parameters:"] # [doc = ""] # [doc = " - name of the ZST representing the field modulus"] # [doc = " - hex serialization of the modulus"] # [doc = " - `crypto-bigint` unsigned integer type (e.g. U256)"] # [doc = " - number of bytes in an encoded field element"] # [doc = " - byte order to use when encoding/decoding field elements"] # [doc = " - documentation string for the field modulus type"] # [doc = ""] # [doc = " ```"] # [doc = " use primefield::{ByteOrder, bigint::U256, consts::U32};"] # [doc = ""] # [doc = " primefield::monty_field_params!("] # [doc = "     name: FieldParams,"] # [doc = "     modulus: \"ffffffff00000001000000000000000000000000ffffffffffffffffffffffff\","] # [doc = "     uint: U256,"] # [doc = "     byte_order: ByteOrder::BigEndian,"] # [doc = "     multiplicative_generator: 6,"] # [doc = "     doc: \"P-256 field modulus\""] # [doc = " );"] # [doc = " ```"] # [macro_export] macro_rules ! monty_field_params { (name : $ name : ident , modulus : $ modulus_hex : expr , uint : $ uint : ty , byte_order : $ byte_order : expr , multiplicative_generator : $ multiplicative_generator : expr , doc : $ doc : expr) => { $ crate :: monty_field_params_with_root_of_unity ! { name : $ name , modulus : $ modulus_hex , uint : $ uint , byte_order : $ byte_order , multiplicative_generator : $ multiplicative_generator , root_of_unity : None , doc : $ doc } } ; }
};
}
