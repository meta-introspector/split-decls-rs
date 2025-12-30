// Generated macro for visit_visitable (macro)
macro_rules! Depcrate_visitvisit_visitable {
() => {
// Module: crate::visit
// Provides: {"visit_visitable"}
// Dependencies: {}
macro_rules ! visit_visitable { ($ visitor : expr , $ ($ expr : expr) ,* $ (,) ?) => { { $ (try_visit ! (Visitable :: visit ($ expr , $ visitor , ())) ;) * } } ; }
};
}
