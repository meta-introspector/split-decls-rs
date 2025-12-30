// Generated macro for impl_574 (impl)
macro_rules! Depcrate_visitimpl_574 {
() => {
// Module: crate::visit
// Provides: {"impl_574"}
// Dependencies: {}
impl < 'a , V : Visitor < 'a > , T > Visitable < 'a , V > for (T ,) where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { self . 0 . visit (visitor , extra) } }
};
}
