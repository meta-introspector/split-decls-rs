// Generated macro for immediate_children (function)
macro_rules! Depcrateimmediate_children {
() => {
// Module: crate
// Provides: {"immediate_children"}
// Dependencies: {}
fn immediate_children < 'tu > (entity : & Entity < 'tu > , mut closure : impl FnMut (Entity < 'tu > , EnteredSpan) ,) { entity . visit_children (| entity , _parent | { let span = debug_span ! ("child" , kind = ? entity . get_kind () , dbg = entity . get_name () ,) . entered () ; closure (entity , span) ; EntityVisitResult :: Continue }) ; }
};
}
