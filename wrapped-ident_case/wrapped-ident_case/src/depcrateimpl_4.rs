// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl RenameRule { # [doc = " Change case of a `PascalCase` variant."] pub fn apply_to_variant < S : AsRef < str > > (& self , variant : S) -> String { let variant = variant . as_ref () ; match * self { None | PascalCase => variant . to_owned () , LowerCase => variant . to_ascii_lowercase () , CamelCase => variant [.. 1] . to_ascii_lowercase () + & variant [1 ..] , SnakeCase => { let mut snake = String :: new () ; for (i , ch) in variant . char_indices () { if i > 0 && ch . is_uppercase () { snake . push ('_') ; } snake . push (ch . to_ascii_lowercase ()) ; } snake } ScreamingSnakeCase => SnakeCase . apply_to_variant (variant) . to_ascii_uppercase () , KebabCase => SnakeCase . apply_to_variant (variant) . replace ('_' , "-") , } } # [doc = " Change case of a `snake_case` field."] pub fn apply_to_field < S : AsRef < str > > (& self , field : S) -> String { let field = field . as_ref () ; match * self { None | LowerCase | SnakeCase => field . to_owned () , PascalCase => { let mut pascal = String :: new () ; let mut capitalize = true ; for ch in field . chars () { if ch == '_' { capitalize = true ; } else if capitalize { pascal . push (ch . to_ascii_uppercase ()) ; capitalize = false ; } else { pascal . push (ch) ; } } pascal } CamelCase => { let pascal = PascalCase . apply_to_field (field) ; pascal [.. 1] . to_ascii_lowercase () + & pascal [1 ..] } ScreamingSnakeCase => field . to_ascii_uppercase () , KebabCase => field . replace ('_' , "-") , } } }
};
}
