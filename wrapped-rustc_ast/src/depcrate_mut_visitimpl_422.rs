// Generated macro for impl_422 (impl)
macro_rules! Depcrate_mut_visitimpl_422 {
() => {
// Module: crate::mut_visit
// Provides: {"impl_422"}
// Dependencies: {}
impl < V : MutVisitor , T > MutVisitable < V > for [T] where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { for item in self { item . visit_mut (visitor , extra) ; } } }
};
}
