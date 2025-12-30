// Generated macro for check_duplicated_attr (function)
macro_rules! Depcrate_attrs_duplicated_attributescheck_duplicated_attr {
() => {
// Module: crate::attrs::duplicated_attributes
// Provides: {"check_duplicated_attr"}
// Dependencies: {}
fn check_duplicated_attr (cx : & EarlyContext < '_ > , attr : & MetaItem , attr_paths : & mut FxHashMap < String , Span > , parent : & mut Vec < Symbol > ,) { if attr . span . from_expansion () { return ; } let attr_path = if let Some (ident) = attr . ident () { ident . name } else { Symbol :: intern (& path_to_string (& attr . path)) } ; if let Some (ident) = attr . ident () { let name = ident . name ; if name == sym :: doc || name == sym :: cfg_attr_trace || name == sym :: rustc_on_unimplemented || name == sym :: reason { return ; } if let Some (direct_parent) = parent . last () && * direct_parent == sym :: cfg_trace && [sym :: all , sym :: not , sym :: any] . contains (& name) { return ; } } if let Some (value) = attr . value_str () { emit_if_duplicated (cx , attr , attr_paths , format ! ("{}:{attr_path}={value}" , parent . iter () . join (":")) ,) ; } else if let Some (sub_attrs) = attr . meta_item_list () { parent . push (attr_path) ; for sub_attr in sub_attrs { if let Some (meta) = sub_attr . meta_item () { check_duplicated_attr (cx , meta , attr_paths , parent) ; } } parent . pop () ; } else { emit_if_duplicated (cx , attr , attr_paths , format ! ("{}:{attr_path}" , parent . iter () . join (":"))) ; } }
};
}
