// Generated macro for collect_args_fields (function)
macro_rules! Depcrate_derives_argscollect_args_fields {
() => {
// Module: crate::derives::args
// Provides: {"collect_args_fields"}
// Dependencies: {}
pub (crate) fn collect_args_fields < 'a > (item : & 'a Item , fields : & 'a FieldsNamed ,) -> Result < Vec < (& 'a Field , Item) > , syn :: Error > { fields . named . iter () . map (| field | { let item = Item :: from_args_field (field , item . casing () , item . env_casing ()) ? ; Ok ((field , item)) }) . collect () }
};
}
