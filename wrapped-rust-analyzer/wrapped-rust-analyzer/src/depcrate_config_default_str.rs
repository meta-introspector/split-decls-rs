// Generated macro for _default_str (macro)
macro_rules! Depcrate_config_default_str {
() => {
// Module: crate::config
// Provides: {"_default_str"}
// Dependencies: {}
macro_rules ! _default_str { ($ default : expr , $ ty : ty) => { { let val = default_val ! ($ default , $ ty) ; serde_json :: to_string_pretty (& val) . unwrap () } } ; }
};
}
