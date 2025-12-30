// Generated macro for to_stdvec_dyn (function)
macro_rules! Depcrate_serto_stdvec_dyn {
() => {
// Module: crate::ser
// Provides: {"to_stdvec_dyn"}
// Dependencies: {}
pub fn to_stdvec_dyn (schema : & OwnedNamedType , value : & Value) -> Result < Vec < u8 > , Error > { let mut out = vec ! [] ; ser_named_type (& schema . ty , value , & mut out) ? ; Ok (out) }
};
}
