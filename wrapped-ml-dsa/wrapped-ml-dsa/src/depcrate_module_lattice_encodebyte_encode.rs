// Generated macro for byte_encode (function)
macro_rules! Depcrate_module_lattice_encodebyte_encode {
() => {
// Module: crate::module_lattice::encode
// Provides: {"byte_encode"}
// Dependencies: {}
fn byte_encode < F : Field , D : EncodingSize > (vals : & DecodedValue < F >) -> EncodedPolynomial < D > { let val_step = D :: ValueStep :: USIZE ; let byte_step = D :: ByteStep :: USIZE ; let mut bytes = EncodedPolynomial :: < D > :: default () ; let vc = vals . chunks (val_step) ; let bc = bytes . chunks_mut (byte_step) ; for (v , b) in vc . zip (bc) { let mut x = 0u128 ; for (j , vj) in v . iter () . enumerate () { let vj : u128 = vj . 0 . into () ; x |= vj << (D :: USIZE * j) ; } let xb = x . to_le_bytes () ; b . copy_from_slice (& xb [.. byte_step]) ; } bytes }
};
}
