// Generated macro for bridged_to (function)
macro_rules! Depcrate_stmtbridged_to {
() => {
// Module: crate::stmt
// Provides: {"bridged_to"}
// Dependencies: {}
# [doc = " Whether the entity contains a bridging modifier, and if so, what that"] # [doc = " modifier bridges to."] pub (crate) fn bridged_to (entity : & Entity < '_ > , context : & Context < '_ >) -> Option < Option < String > > { let mut bridged = None ; immediate_children (entity , | child , _span | { if let EntityKind :: UnexposedAttr = child . get_kind () { if let Some (attr) = UnexposedAttr :: parse (& child , context) { match attr { UnexposedAttr :: Bridged (to) | UnexposedAttr :: BridgedMutable (to) => { if to == "id" { bridged = Some (None) ; } else { bridged = Some (Some (to)) ; } } UnexposedAttr :: BridgedRelated | UnexposedAttr :: BridgedTypedef | UnexposedAttr :: BridgedImplicit => { bridged = Some (None) ; } _ => { } } } } }) ; bridged }
};
}
