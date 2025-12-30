// Generated macro for impl_101 (impl)
macro_rules! Depcrate_parseimpl_101 {
() => {
// Module: crate::parse
// Provides: {"impl_101"}
// Dependencies: {}
impl LinkStack { fn push (& mut self , el : LinkStackEl) { self . inner . push (el) ; } fn pop (& mut self) -> Option < LinkStackEl > { let el = self . inner . pop () ; self . disabled_ix = core :: cmp :: min (self . disabled_ix , self . inner . len ()) ; el } fn clear (& mut self) { self . inner . clear () ; self . disabled_ix = 0 ; } fn disable_all_links (& mut self) { for el in & mut self . inner [self . disabled_ix ..] { if el . ty == LinkStackTy :: Link { el . ty = LinkStackTy :: Disabled ; } } self . disabled_ix = self . inner . len () ; } }
};
}
