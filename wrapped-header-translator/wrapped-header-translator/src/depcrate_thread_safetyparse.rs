// Generated macro for parse (function)
macro_rules! Depcrate_thread_safetyparse {
() => {
// Module: crate::thread_safety
// Provides: {"parse"}
// Dependencies: {}
fn parse (entity : & Entity < '_ > , context : & Context < '_ >) -> (Option < bool > , bool) { let mut sendable = None ; let mut mainthreadonly = false ; immediate_children (entity , | entity , _span | { if let EntityKind :: UnexposedAttr = entity . get_kind () { if let Some (attr) = UnexposedAttr :: parse (& entity , context) { match attr { UnexposedAttr :: Sendable => { sendable = Some (true) ; } UnexposedAttr :: NonSendable => { sendable = Some (false) ; } UnexposedAttr :: UIActor => { mainthreadonly = true ; } _ => { } } } } }) ; (sendable , mainthreadonly) }
};
}
