// Generated macro for impl_293 (impl)
macro_rules! Depcrate_protocolimpl_293 {
() => {
// Module: crate::protocol
// Provides: {"impl_293"}
// Dependencies: {}
impl ProtocolRef { pub (crate) fn super_protocols (entity : & Entity < '_ > , context : & Context < '_ >) -> Vec < Self > { let mut super_protocols = Vec :: new () ; for entity in parse_direct_protocols (entity , context) { super_protocols . push (ProtocolRef :: from_entity (& entity , context)) ; } super_protocols } pub (crate) fn from_entity (entity : & Entity < '_ > , context : & Context < '_ >) -> Self { let id = ItemIdentifier :: new (entity , context) ; let mut super_protocols = Vec :: new () ; for entity in parse_direct_protocols (entity , context) { super_protocols . push (ProtocolRef :: from_entity (& entity , context)) ; } Self { id , super_protocols , } } pub (crate) fn required_items (& self) -> impl IntoIterator < Item = ItemTree > { [ItemTree :: new (self . id . clone () , self . super_protocols . iter () . flat_map (| super_protocol | super_protocol . required_items ()) ,) , ItemTree :: objc ("__macros__") ,] } pub (crate) fn is_subprotocol_of (& self , protocol_name : & str) -> bool { if self . id . name == protocol_name { return true ; } for p in & self . super_protocols { if p . is_subprotocol_of (protocol_name) { return true ; } } false } }
};
}
