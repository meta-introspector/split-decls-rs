// Generated macro for impl_4012 (impl)
macro_rules! Depcrate_macro_useimpl_4012 {
() => {
// Module: crate::macro_use
// Provides: {"impl_4012"}
// Dependencies: {}
impl MacroUseImports { fn push_unique_macro (& mut self , cx : & LateContext < '_ > , span : Span) { let call_site = span . source_callsite () ; let name = snippet (cx , cx . sess () . source_map () . span_until_char (call_site , '!') , "_") ; if span . source_callee () . is_some () && ! self . collected . contains (& call_site) { let name = if name . contains ("::") { name . split ("::") . last () . unwrap () . to_string () } else { name . to_string () } ; self . mac_refs . push (MacroRefData :: new (name)) ; self . collected . insert (call_site) ; } } fn push_unique_macro_pat_ty (& mut self , cx : & LateContext < '_ > , span : Span) { let call_site = span . source_callsite () ; let name = snippet (cx , cx . sess () . source_map () . span_until_char (call_site , '!') , "_") ; if span . source_callee () . is_some () && ! self . collected . contains (& call_site) { self . mac_refs . push (MacroRefData :: new (name . to_string ())) ; self . collected . insert (call_site) ; } } }
};
}
