// Generated macro for impl_419 (impl)
macro_rules! Depcrate_mut_visitimpl_419 {
() => {
// Module: crate::mut_visit
// Provides: {"impl_419"}
// Dependencies: {}
impl < V : MutVisitor , T : ? Sized > MutVisitable < V > for Box < T > where T : MutVisitable < V > , { type Extra = T :: Extra ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { (* * self) . visit_mut (visitor , extra) } }
};
}
