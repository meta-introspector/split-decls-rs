// Generated macro for impl_573 (impl)
macro_rules! Depcrate_visitimpl_573 {
() => {
// Module: crate::visit
// Provides: {"impl_573"}
// Dependencies: {}
impl < 'a , V : Visitor < 'a > , T > Visitable < 'a , V > for Vec < T > where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { for item in self { try_visit ! (item . visit (visitor , extra)) ; } V :: Result :: output () } }
};
}
