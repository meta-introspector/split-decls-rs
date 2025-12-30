// Generated macro for visit_opt (macro)
macro_rules! Depcrate_visitvisit_opt {
() => {
// Module: crate::visit
// Provides: {"visit_opt"}
// Dependencies: {}
# [macro_export] macro_rules ! visit_opt { ($ visitor : expr , $ method : ident , $ opt : expr $ (, $ ($ extra_args : expr) ,*) ?) => { if let Some (x) = $ opt { $ crate :: try_visit ! ($ visitor .$ method (x $ (, $ ($ extra_args ,) *) ?)) ; } } }
};
}
