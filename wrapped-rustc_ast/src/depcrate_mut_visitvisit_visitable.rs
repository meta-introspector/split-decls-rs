// Generated macro for visit_visitable (macro)
macro_rules! Depcrate_mut_visitvisit_visitable {
() => {
// Module: crate::mut_visit
// Provides: {"visit_visitable"}
// Dependencies: {}
macro_rules ! visit_visitable { (mut $ visitor : expr , $ ($ expr : expr) ,* $ (,) ?) => { { $ (MutVisitable :: visit_mut ($ expr , $ visitor , ()) ;) * } } ; }
};
}
