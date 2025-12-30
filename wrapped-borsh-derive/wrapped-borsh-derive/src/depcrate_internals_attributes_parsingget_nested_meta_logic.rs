// Generated macro for get_nested_meta_logic (function)
macro_rules! Depcrate_internals_attributes_parsingget_nested_meta_logic {
() => {
// Module: crate::internals::attributes::parsing
// Provides: {"get_nested_meta_logic"}
// Dependencies: {}
fn get_nested_meta_logic < T , F > (attr_name : Symbol , meta : ParseNestedMeta , map : & BTreeMap < Symbol , F > , result : & mut BTreeMap < Symbol , T > ,) -> syn :: Result < () > where F : Fn (Symbol , Symbol , & ParseNestedMeta) -> syn :: Result < T > , { let mut match_ = false ; for (symbol_key , func) in map . iter () { if meta . path == * symbol_key { let v = func (attr_name , * symbol_key , & meta) ? ; result . insert (* symbol_key , v) ; match_ = true ; } } if ! match_ { let keys_strs = map . keys () . map (| symbol | symbol . 1) . collect :: < Vec < _ > > () ; let keys_strs = keys_strs . join (", ") ; return Err (meta . error (format_args ! ("malformed {0} attribute, expected `{0}({1})`" , attr_name . 0 , keys_strs))) ; } Ok (()) }
};
}
