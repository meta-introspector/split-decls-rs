// Generated macro for impl_420 (impl)
macro_rules! Depcrate_mut_visitimpl_420 {
() => {
// Module: crate::mut_visit
// Provides: {"impl_420"}
// Dependencies: {}
impl < V : MutVisitor , T > MutVisitable < V > for Option < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { if let Some (this) = self { this . visit_mut (visitor , extra) } } }
};
}
