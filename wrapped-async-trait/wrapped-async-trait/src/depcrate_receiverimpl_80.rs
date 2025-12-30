// Generated macro for impl_80 (impl)
macro_rules! Depcrate_receiverimpl_80 {
() => {
// Module: crate::receiver
// Provides: {"impl_80"}
// Dependencies: {}
impl VisitMut for ReplaceSelf { fn visit_ident_mut (& mut self , i : & mut Ident) { prepend_underscore_to_self (i) ; } fn visit_path_mut (& mut self , p : & mut Path) { if p . segments . len () == 1 { self . visit_ident_mut (& mut p . segments [0] . ident) ; } for segment in & mut p . segments { self . visit_path_arguments_mut (& mut segment . arguments) ; } } fn visit_item_mut (& mut self , i : & mut Item) { if let Item :: Macro (i) = i { if i . mac . path . is_ident ("macro_rules") || i . mac . path . segments . last () . unwrap () . ident == "select" { self . visit_macro_mut (& mut i . mac) ; } } } fn visit_macro_mut (& mut self , mac : & mut Macro) { if ! contains_fn (mac . tokens . clone ()) { self . visit_token_stream (& mut mac . tokens) ; } } }
};
}
