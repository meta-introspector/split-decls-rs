// Generated macro for impl_53 (impl)
macro_rules! Depcrate_typesimpl_53 {
() => {
// Module: crate::types
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a > Demarshal < 'a > for & 'a ObjectPath { fn read_buf (b : & mut DemarshalState < 'a >) -> Result < Self , DemarshalError > { let r = b . read_str (b's') ? ; Ok (ObjectPath :: new (r) . map_err (| _ | DemarshalError :: InvalidString) ?) } }
};
}
