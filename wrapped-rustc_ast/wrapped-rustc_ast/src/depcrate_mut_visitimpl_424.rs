// Generated macro for impl_424 (impl)
macro_rules! Depcrate_mut_visitimpl_424 {
() => {
// Module: crate::mut_visit
// Provides: {"impl_424"}
// Dependencies: {}
impl < V : MutVisitor , T > MutVisitable < V > for (T ,) where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { self . 0 . visit_mut (visitor , extra) ; } }
};
}
