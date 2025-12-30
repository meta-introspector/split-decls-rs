// Generated macro for walk_list (macro)
macro_rules! Depcrate_visitwalk_list {
() => {
// Module: crate::visit
// Provides: {"walk_list"}
// Dependencies: {}
# [macro_export] macro_rules ! walk_list { ($ visitor : expr , $ method : ident , $ list : expr $ (, $ ($ extra_args : expr) ,*) ?) => { for elem in $ list { $ crate :: try_visit ! ($ visitor .$ method (elem $ (, $ ($ extra_args ,) *) ?)) ; } } }
};
}
