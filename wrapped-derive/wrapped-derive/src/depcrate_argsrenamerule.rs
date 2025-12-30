// Generated macro for RenameRule (enum)
macro_rules! Depcrate_argsRenameRule {
() => {
// Module: crate::args
// Provides: {"RenameRule"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , FromMeta)] pub enum RenameRule { # [darling (rename = "lowercase")] Lower , # [darling (rename = "UPPERCASE")] Upper , # [darling (rename = "PascalCase")] Pascal , # [darling (rename = "camelCase")] Camel , # [darling (rename = "snake_case")] Snake , # [darling (rename = "SCREAMING_SNAKE_CASE")] ScreamingSnake , }
};
}
