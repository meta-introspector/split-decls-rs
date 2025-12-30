// Generated macro for impl_569 (impl)
macro_rules! Depcrate_visitimpl_569 {
() => {
// Module: crate::visit
// Provides: {"impl_569"}
// Dependencies: {}
impl < 'a , V : Visitor < 'a > , T : ? Sized > Visitable < 'a , V > for Box < T > where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { (* * self) . visit (visitor , extra) } }
};
}
