// Generated macro for impl_571 (impl)
macro_rules! Depcrate_visitimpl_571 {
() => {
// Module: crate::visit
// Provides: {"impl_571"}
// Dependencies: {}
impl < 'a , V : Visitor < 'a > , T > Visitable < 'a , V > for Spanned < T > where T : Visitable < 'a , V > , { type Extra = T :: Extra ; fn visit (& 'a self , visitor : & mut V , extra : Self :: Extra) -> V :: Result { let Spanned { span : _ , node } = self ; node . visit (visitor , extra) } }
};
}
