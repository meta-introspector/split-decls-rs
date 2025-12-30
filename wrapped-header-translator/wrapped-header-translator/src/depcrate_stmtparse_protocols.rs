// Generated macro for parse_protocols (function)
macro_rules! Depcrate_stmtparse_protocols {
() => {
// Module: crate::stmt
// Provides: {"parse_protocols"}
// Dependencies: {}
# [doc = " Find all protocols, protocol's protocols and superclass' protocols."] fn parse_protocols < 'tu > (entity : & Entity < 'tu > , protocols : & mut BTreeMap < ItemIdentifier , Entity < 'tu > > , context : & Context < '_ > ,) { immediate_children (entity , | entity , _span | match entity . get_kind () { EntityKind :: ObjCProtocolRef => { let entity = entity . get_reference () . expect ("ObjCProtocolRef to reference entity") ; if protocols . insert (ItemIdentifier :: new (& entity , context) , entity) . is_none () { parse_protocols (& entity , protocols , context) ; } } EntityKind :: ObjCSuperClassRef => { let entity = entity . get_reference () . expect ("ObjCSuperClassRef to reference entity") ; parse_protocols (& entity , protocols , context) ; } _ => { } }) ; }
};
}
