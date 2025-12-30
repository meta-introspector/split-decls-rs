// Generated macro for interface_signature (function)
macro_rules! Depcrate_typesinterface_signature {
() => {
// Module: crate::types
// Provides: {"interface_signature"}
// Dependencies: {}
pub fn interface_signature (def : TypeDef , generics : & [Type]) -> String { if generics . is_empty () { let guid = def . guid_attribute () . unwrap () ; format ! ("{{{guid}}}") } else { let guid = def . guid_attribute () . unwrap () ; let mut signature = format ! ("pinterface({{{guid}}}") ; for generic in generics { signature . push (';') ; signature . push_str (& generic . runtime_signature ()) } signature . push (')') ; signature } }
};
}
