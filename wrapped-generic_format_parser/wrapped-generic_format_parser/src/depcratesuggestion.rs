// Generated macro for Suggestion (enum)
macro_rules! DepcrateSuggestion {
() => {
// Module: crate
// Provides: {"Suggestion"}
// Dependencies: {}
pub enum Suggestion { None , # [doc = " Replace inline argument with positional argument:"] # [doc = " `format!(\"{foo.bar}\")` -> `format!(\"{}\", foo.bar)`"] UsePositional , # [doc = " Remove `r#` from identifier:"] # [doc = " `format!(\"{r#foo}\")` -> `format!(\"{foo}\")`"] RemoveRawIdent (InnerSpan) , }
};
}
