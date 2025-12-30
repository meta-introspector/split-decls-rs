// Generated macro for generate_fixed_input (function)
macro_rules! Depcrate_backend_kdfgenerate_fixed_input {
() => {
// Module: crate::backend::kdf
// Provides: {"generate_fixed_input"}
// Dependencies: {}
fn generate_fixed_input (py : pyo3 :: Python < '_ > , length : usize , params : & KbkdfParams ,) -> CryptographyResult < Vec < u8 > > { if let Some (ref fixed_data) = params . fixed { return Ok (fixed_data . as_bytes (py) . to_vec ()) ; } let py_bitlength = pyo3 :: types :: PyInt :: new (py , length . checked_mul (8) . ok_or (pyo3 :: exceptions :: PyOverflowError :: new_err ("Length too large, would cause overflow in bit length calculation" ,)) ? ,) ; let l_val = py_uint_to_be_bytes_with_length (py , py_bitlength , params . llen . unwrap ()) ? ; let mut result = Vec :: new () ; let label : & [u8] = params . label . as_ref () . map_or (b"" , | l | l . as_bytes (py)) ; result . extend_from_slice (label) ; result . push (0x00) ; let context : & [u8] = params . context . as_ref () . map_or (b"" , | l | l . as_bytes (py)) ; result . extend_from_slice (context) ; result . extend_from_slice (l_val . as_ref ()) ; Ok (result) }
};
}
