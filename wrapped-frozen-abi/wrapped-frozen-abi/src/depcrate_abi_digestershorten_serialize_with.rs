// Generated macro for shorten_serialize_with (function)
macro_rules! Depcrate_abi_digestershorten_serialize_with {
() => {
// Module: crate::abi_digester
// Provides: {"shorten_serialize_with"}
// Dependencies: {}
pub (crate) fn shorten_serialize_with (type_name : & str) -> & str { if type_name . ends_with ("__SerializeWith") { "__SerializeWith" } else { type_name } }
};
}
