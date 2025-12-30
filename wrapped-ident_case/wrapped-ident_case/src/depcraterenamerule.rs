// Generated macro for RenameRule (enum)
macro_rules! DepcrateRenameRule {
() => {
// Module: crate
// Provides: {"RenameRule"}
// Dependencies: {}
# [doc = " A casing rule for renaming Rust identifiers."] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum RenameRule { # [doc = " No-op rename rule."] None , # [doc = " Rename direct children to \"lowercase\" style."] LowerCase , # [doc = " Rename direct children to \"PascalCase\" style, as typically used for enum variants."] PascalCase , # [doc = " Rename direct children to \"camelCase\" style."] CamelCase , # [doc = " Rename direct children to \"snake_case\" style, as commonly used for fields."] SnakeCase , # [doc = " Rename direct children to \"SCREAMING_SNAKE_CASE\" style, as commonly used for constants."] ScreamingSnakeCase , # [doc = " Rename direct children to \"kebab-case\" style."] KebabCase , }
};
}
