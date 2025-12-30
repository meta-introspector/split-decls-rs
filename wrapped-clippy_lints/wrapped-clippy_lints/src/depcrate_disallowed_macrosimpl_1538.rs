// Generated macro for impl_1538 (impl)
macro_rules! Depcrate_disallowed_macrosimpl_1538 {
() => {
// Module: crate::disallowed_macros
// Provides: {"impl_1538"}
// Dependencies: {}
impl DisallowedMacros { pub fn new (tcx : TyCtxt < '_ > , conf : & 'static Conf , early_macro_cache : AttrStorage) -> Self { let (disallowed , _) = create_disallowed_map (tcx , & conf . disallowed_macros , PathNS :: Macro , | def_kind | matches ! (def_kind , DefKind :: Macro (_)) , "macro" , false ,) ; Self { disallowed , seen : FxHashSet :: default () , derive_src : None , early_macro_cache , } } fn check (& mut self , cx : & LateContext < '_ > , span : Span , derive_src : Option < OwnerId >) { if self . disallowed . is_empty () { return ; } for mac in macro_backtrace (span) { if ! self . seen . insert (mac . expn) { return ; } if let Some (& (path , disallowed_path)) = self . disallowed . get (& mac . def_id) { let msg = format ! ("use of a disallowed macro `{path}`") ; let add_note = disallowed_path . diag_amendment (mac . span) ; if matches ! (mac . kind , MacroKind :: Derive) && let Some (derive_src) = derive_src { span_lint_hir_and_then (cx , DISALLOWED_MACROS , cx . tcx . local_def_id_to_hir_id (derive_src . def_id) , mac . span , msg , add_note ,) ; } else { span_lint_and_then (cx , DISALLOWED_MACROS , mac . span , msg , add_note) ; } } } } }
};
}
