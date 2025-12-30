// Generated macro for canonical_script (function)
macro_rules! Depcrate_unicodecanonical_script {
() => {
// Module: crate::unicode
// Provides: {"canonical_script"}
// Dependencies: {}
fn canonical_script (normalized_value : & str ,) -> Result < Option < & 'static str > , Error > { let scripts = property_values ("Script") ? . unwrap () ; Ok (canonical_value (scripts , normalized_value)) }
};
}
