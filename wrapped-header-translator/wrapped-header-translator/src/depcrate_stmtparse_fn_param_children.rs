// Generated macro for parse_fn_param_children (function)
macro_rules! Depcrate_stmtparse_fn_param_children {
() => {
// Module: crate::stmt
// Provides: {"parse_fn_param_children"}
// Dependencies: {}
fn parse_fn_param_children (parent : & Entity < '_ > , context : & Context < '_ >) -> Option < UnexposedAttr > { let mut ret = None ; immediate_children (parent , | entity , _span | match entity . get_kind () { EntityKind :: UnexposedAttr => { if let Some (attr) = UnexposedAttr :: parse (& entity , context) { if ret . is_some () { error ! ("found multiple attributes {ret:?} and {attr:?} on fn param") ; } ret = Some (attr) ; } } EntityKind :: ObjCClassRef | EntityKind :: TypeRef | EntityKind :: ObjCProtocolRef | EntityKind :: ParmDecl => { } EntityKind :: NSConsumed => { error ! ("found NSConsumed, which requires manual handling") ; } EntityKind :: IntegerLiteral => { } kind => error ! (? parent , ? kind , "unknown") , }) ; ret }
};
}
