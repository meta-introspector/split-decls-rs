// Generated macro for default_text (macro)
macro_rules! Depcrate_confdefault_text {
() => {
// Module: crate::conf
// Provides: {"default_text"}
// Dependencies: {}
macro_rules ! default_text { ($ value : expr) => { { let mut text = String :: new () ; $ value . serialize (toml :: ser :: ValueSerializer :: new (& mut text)) . unwrap () ; text } } ; ($ value : expr , $ override : expr) => { $ override . to_string () } ; }
};
}
