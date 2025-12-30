// Generated macro for impl_58 (impl)
macro_rules! Depcrate_typesimpl_58 {
() => {
// Module: crate::types
// Provides: {"impl_58"}
// Dependencies: {}
impl Demarshal < '_ > for u32 { fn read_buf (b : & mut DemarshalState < '_ >) -> Result < Self , DemarshalError > { let x = b . read_single (4 , 4) ? ; let x : [u8 ; 4] = x . try_into () . unwrap () ; let z = (if b . is_big_endian { u32 :: from_be_bytes (x) } else { u32 :: from_le_bytes (x) }) as u32 ; Ok (z) } }
};
}
