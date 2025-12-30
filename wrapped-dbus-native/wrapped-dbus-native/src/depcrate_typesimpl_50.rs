// Generated macro for impl_50 (impl)
macro_rules! Depcrate_typesimpl_50 {
() => {
// Module: crate::types
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a > Demarshal < 'a > for & 'a Str { fn read_buf (b : & mut DemarshalState < 'a >) -> Result < Self , DemarshalError > { let r = b . read_str (b's') ? ; Ok (Str :: new (r) . map_err (| _ | DemarshalError :: InvalidString) ?) } }
};
}
