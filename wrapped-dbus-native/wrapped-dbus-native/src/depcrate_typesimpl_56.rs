// Generated macro for impl_56 (impl)
macro_rules! Depcrate_typesimpl_56 {
() => {
// Module: crate::types
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a > Demarshal < 'a > for & 'a Signature { fn read_buf (b : & mut DemarshalState < 'a >) -> Result < Self , DemarshalError > { let z = u8 :: read_buf (b) ? as usize ; let new_pos = b . pos + z + 1 ; if new_pos > b . buf . len () { Err (DemarshalError :: NotEnoughData) ? } ; let r = & b . buf [b . pos .. b . pos + z] ; let r = std :: str :: from_utf8 (r) . map_err (| _ | DemarshalError :: InvalidString) ? ; b . pos = new_pos ; let r = Signature :: new (r) . map_err (| _ | DemarshalError :: InvalidString) ? ; Ok (r) } }
};
}
