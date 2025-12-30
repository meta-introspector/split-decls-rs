// Generated macro for default_translator (function)
macro_rules! Depcratedefault_translator {
() => {
// Module: crate
// Provides: {"default_translator"}
// Dependencies: {}
pub fn default_translator () -> Translator { Translator :: with_fallback_bundle (DEFAULT_LOCALE_RESOURCES . to_vec () , false) }
};
}
