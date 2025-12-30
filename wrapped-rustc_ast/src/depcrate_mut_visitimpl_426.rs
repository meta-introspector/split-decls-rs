// Generated macro for impl_426 (impl)
macro_rules! Depcrate_mut_visitimpl_426 {
() => {
// Module: crate::mut_visit
// Provides: {"impl_426"}
// Dependencies: {}
impl < V : MutVisitor , T1 , T2 , T3 > MutVisitable < V > for (T1 , T2 , T3) where T1 : MutVisitable < V , Extra = () > , T2 : MutVisitable < V , Extra = () > , T3 : MutVisitable < V , Extra = () > , { type Extra = () ; fn visit_mut (& mut self , visitor : & mut V , extra : Self :: Extra) { self . 0 . visit_mut (visitor , extra) ; self . 1 . visit_mut (visitor , extra) ; self . 2 . visit_mut (visitor , extra) ; } }
};
}
