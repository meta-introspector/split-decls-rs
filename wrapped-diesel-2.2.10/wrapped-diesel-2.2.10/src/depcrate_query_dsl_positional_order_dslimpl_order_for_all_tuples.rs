// Generated macro for impl_order_for_all_tuples (macro)
macro_rules! Depcrate_query_dsl_positional_order_dslimpl_order_for_all_tuples {
() => {
// Module: crate::query_dsl::positional_order_dsl
// Provides: {"impl_order_for_all_tuples"}
// Dependencies: {}
macro_rules ! impl_order_for_all_tuples { ($ ($ unused1 : tt { $ (($ idx : tt) -> $ T : ident , $ unused2 : ident , $ unused3 : tt ,) + }) +) => { $ (impl <$ ($ T : Order) ,+> Order for ($ ($ T ,) +) { type Fragment = ($ (<$ T as Order >:: Fragment ,) +) ; fn into_fragment (self) -> Self :: Fragment { ($ (self .$ idx . into_fragment () ,) +) } }) + } ; }
};
}
