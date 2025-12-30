// Generated macro for parse_protocol (function)
macro_rules! Depcrate_rust_typeparse_protocol {
() => {
// Module: crate::rust_type
// Provides: {"parse_protocol"}
// Dependencies: {}
fn parse_protocol (entity : Entity < '_ > , context : & Context < '_ >) -> (ProtocolRef , ThreadSafety) { let entity = entity . get_definition () . unwrap_or (entity) ; let entity = entity . get_location () . expect ("itemref location") . get_entity () . expect ("itemref entity") ; let id = ItemIdentifier :: new (& entity , context) ; match entity . get_kind () { EntityKind :: ObjCProtocolDecl => { let protocol = ProtocolRef :: from_entity (& entity , context) ; let thread_safety = ThreadSafety :: from_decl (& entity , context) ; (protocol , thread_safety) } EntityKind :: ObjCProtocolRef => { let protocol = ProtocolRef :: from_entity (& entity , context) ; let thread_safety = ThreadSafety :: from_ref (& entity , context) ; (protocol , thread_safety) } _ => { error ! (? entity , "was not a protocol") ; let protocol = ProtocolRef { id , super_protocols : vec ! [] , } ; (protocol , ThreadSafety :: dummy ()) } } }
};
}
