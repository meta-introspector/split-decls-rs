// Generated macro for check_builtin_meta_item (function)
macro_rules! Depcrate_validate_attrcheck_builtin_meta_item {
() => {
// Module: crate::validate_attr
// Provides: {"check_builtin_meta_item"}
// Dependencies: {}
pub fn check_builtin_meta_item (psess : & ParseSess , meta : & MetaItem , style : ast :: AttrStyle , name : Symbol , template : AttributeTemplate , deny_unsafety : bool ,) { if ! is_attr_template_compatible (& template , & meta . kind) { emit_malformed_attribute (psess , style , meta . span , name , template) ; } if deny_unsafety { deny_builtin_meta_unsafety (psess . dcx () , meta . unsafety , & meta . path) ; } }
};
}
