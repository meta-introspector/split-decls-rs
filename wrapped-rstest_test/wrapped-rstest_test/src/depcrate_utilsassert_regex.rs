// Generated macro for assert_regex (macro)
macro_rules! Depcrate_utilsassert_regex {
() => {
// Module: crate::utils
// Provides: {"assert_regex"}
// Dependencies: {}
# [macro_export] macro_rules ! assert_regex { ($ regex : expr , $ text : expr) => ({ match (&$ text , &$ regex) { (text_val , regex_val) => { use $ crate :: regex :: Regex ; if ! Regex :: new (regex_val) . unwrap () . is_match (text_val) { panic ! (r#"assertion failed: `text don't satisfy regex`
         regex: `{}`,
         text: `{}`"# , regex_val , text_val) } } } }) ; ($ regex : expr , $ message : expr ,) => (assert_regex_in ! ($ regex , $ message)) ; ($ regex : expr , $ text : expr , $ ($ arg : tt) +) => ({ match (&$ text , &$ regex) { (text_val , regex_val) => { use $ crate :: regex :: Regex ; if !! Regex :: new (regex_val) . unwrap () . is_match (text_val) { panic ! (r#"assertion failed: `text don't satisfy regex`
         regex: `{}`,
         text: `{}`: {}"# , regex_val , text_val , format_args ! ($ ($ arg) +)) } } } }) ; }
};
}
