// Generated macro for impl_94 (impl)
macro_rules! Depcrate_marshalledimpl_94 {
() => {
// Module: crate::marshalled
// Provides: {"impl_94"}
// Dependencies: {}
impl StructBuf { pub fn new (inner : MultiBuf) -> Result < Self , DemarshalError > { let mut outer_sig = String :: with_capacity (inner . sig . len () + 2) ; outer_sig . push ('(') ; outer_sig . push_str (& inner . sig) ; outer_sig . push (')') ; let outer_sig = SignatureSingle :: new_owned (outer_sig) . map_err (| _ | DemarshalError :: InvalidString) ? ; Ok (StructBuf { inner , outer_sig }) } }
};
}
