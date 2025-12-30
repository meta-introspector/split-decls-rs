// Generated macro for visit_visitable_with (macro)
macro_rules! Depcrate_visitvisit_visitable_with {
() => {
// Module: crate::visit
// Provides: {"visit_visitable_with"}
// Dependencies: {}
macro_rules ! visit_visitable_with { ($ visitor : expr , $ expr : expr , $ extra : expr $ (,) ?) => { try_visit ! (Visitable :: visit ($ expr , $ visitor , $ extra)) } ; }
};
}
