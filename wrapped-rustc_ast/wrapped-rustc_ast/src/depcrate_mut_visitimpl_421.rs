// Generated macro for impl_421 (impl)
macro_rules! Depcrate_mut_visitimpl_421 {
() => {
// Module: crate::mut_visit
// Provides: {"impl_421"}
// Dependencies: {}
impl < V : MutVisitor , T > MutVisitable < V > for Spanned < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { let Spanned { span , node } = self ; span . visit_mut (visitor , ()) ; node . visit_mut (visitor , extra) ; } }
};
}
