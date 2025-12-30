// Generated macro for canonical_gencat (function)
macro_rules! Depcrate_unicodecanonical_gencat {
() => {
// Module: crate::unicode
// Provides: {"canonical_gencat"}
// Dependencies: {}
fn canonical_gencat (normalized_value : & str ,) -> Result < Option < & 'static str > , Error > { Ok (match normalized_value { "any" => Some ("Any") , "assigned" => Some ("Assigned") , "ascii" => Some ("ASCII") , _ => { let gencats = property_values ("General_Category") ? . unwrap () ; canonical_value (gencats , normalized_value) } }) }
};
}
