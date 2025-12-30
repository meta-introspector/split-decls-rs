// Generated macro for impl_60 (impl)
macro_rules! Depcrate_typesimpl_60 {
() => {
// Module: crate::types
// Provides: {"impl_60"}
// Dependencies: {}
impl Demarshal < '_ > for u8 { fn read_buf (b : & mut DemarshalState < '_ >) -> Result < Self , DemarshalError > { if b . finished () { Err (DemarshalError :: NotEnoughData) ? } ; let r = b . buf [b . pos] ; b . pos += 1 ; Ok (r) } }
};
}
