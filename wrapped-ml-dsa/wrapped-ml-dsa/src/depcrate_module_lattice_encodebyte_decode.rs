// Generated macro for byte_decode (function)
macro_rules! Depcrate_module_lattice_encodebyte_decode {
() => {
// Module: crate::module_lattice::encode
// Provides: {"byte_decode"}
// Dependencies: {}
fn byte_decode < F : Field , D : EncodingSize > (bytes : & EncodedPolynomial < D >) -> DecodedValue < F > { let val_step = D :: ValueStep :: USIZE ; let byte_step = D :: ByteStep :: USIZE ; let mask = (F :: Int :: one () << D :: USIZE) - F :: Int :: one () ; let mut vals = DecodedValue :: default () ; let vc = vals . chunks_mut (val_step) ; let bc = bytes . chunks (byte_step) ; for (v , b) in vc . zip (bc) { let mut xb = [0u8 ; 16] ; xb [.. byte_step] . copy_from_slice (b) ; let x = u128 :: from_le_bytes (xb) ; for (j , vj) in v . iter_mut () . enumerate () { let val = F :: Int :: truncate (x >> (D :: USIZE * j)) ; vj . 0 = val & mask ; if D :: USIZE == 12 { vj . 0 = vj . 0 % F :: Q ; } } } vals }
};
}
