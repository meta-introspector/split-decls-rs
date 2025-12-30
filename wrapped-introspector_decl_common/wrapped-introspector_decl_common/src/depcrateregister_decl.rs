// Generated macro for register_decl (function)
macro_rules! Depcrateregister_decl {
() => {
// Module: crate
// Provides: {"register_decl"}
// Dependencies: {}
pub fn register_decl (info : DeclInfo) { if let Ok (mut registry) = DECL_REGISTRY . lock () { let idx = registry . declarations . len () ; registry . by_type . entry (info . node_type . to_string ()) . or_default () . push (idx) ; registry . by_module . entry (info . module . to_string ()) . or_default () . push (idx) ; registry . by_hash . insert (info . hash . to_string () , idx) ; registry . declarations . push (info) ; } }
};
}
