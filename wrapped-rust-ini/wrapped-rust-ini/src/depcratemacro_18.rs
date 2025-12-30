// Generated macro for macro_18 (macro)
macro_rules! Depcratemacro_18 {
() => {
// Module: crate
// Provides: {"macro_18"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "case-insensitive")] { # [doc = " Internal storage of section's key"] pub type SectionKey = Option < UniCase < String >>; # [doc = " Internal storage of property's key"] pub type PropertyKey = UniCase < String >; macro_rules ! property_get_key { ($ s : expr) => { & UniCase :: from ($ s) } ; } macro_rules ! property_insert_key { ($ s : expr) => { UniCase :: from ($ s) } ; } macro_rules ! section_key { ($ s : expr) => { $ s . map (| s | UniCase :: from (s . into ())) } ; } } else { # [doc = " Internal storage of section's key"] pub type SectionKey = Option < String >; # [doc = " Internal storage of property's key"] pub type PropertyKey = String ; macro_rules ! property_get_key { ($ s : expr) => { $ s } ; } macro_rules ! property_insert_key { ($ s : expr) => { $ s } ; } macro_rules ! section_key { ($ s : expr) => { $ s . map (Into :: into) } ; } } }
};
}
