// Generated macro for impl_423 (impl)
macro_rules! Depcrate_mut_visitimpl_423 {
() => {
// Module: crate::mut_visit
// Provides: {"impl_423"}
// Dependencies: {}
impl < V : MutVisitor , T > MutVisitable < V > for Vec < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { for item in self { item . visit_mut (visitor , extra) ; } } }
};
}
