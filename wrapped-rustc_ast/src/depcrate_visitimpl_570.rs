// Generated macro for impl_570 (impl)
macro_rules! Depcrate_visitimpl_570 {
() => {
// Module: crate::visit
// Provides: {"impl_570"}
// Dependencies: {}
impl < 'a , V : Visitor < 'a > , T > Visitable < 'a , V > for Option < T > where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { if let Some (this) = self { try_visit ! (this . visit (visitor , extra)) ; } V :: Result :: output () } }
};
}
