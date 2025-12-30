// Generated macro for impl_visitable_list (macro)
macro_rules! Depcrate_mut_visitimpl_visitable_list {
() => {
// Module: crate::mut_visit
// Provides: {"impl_visitable_list"}
// Dependencies: {}
macro_rules ! impl_visitable_list { (< mut > $ ($ ty : ty ,) *) => { $ (impl < V : MutVisitor , T > MutVisitable < V > for $ ty where for <'a > &'a mut $ ty : IntoIterator < Item = &'a mut T >, T : MutVisitable < V >, { type Extra = < T as MutVisitable < V >>:: Extra ; # [inline] fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { for i in self { i . visit_mut (visitor , extra) ; } } }) * } }
};
}
