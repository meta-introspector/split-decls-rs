// Generated macro for assert_in (macro)
macro_rules! Depcrate_utilsassert_in {
() => {
// Module: crate::utils
// Provides: {"assert_in"}
// Dependencies: {}
# [macro_export] macro_rules ! assert_in { ($ text : expr , $ message : expr) => ({ match (&$ text , &$ message) { (text_val , message_val) => { if ! text_val . contains (message_val) { panic ! (r#"assertion failed: `text don't contain message`
         text: `{}`,
         message: `{}`"# , text_val , message_val) } } } }) ; ($ text : expr , $ message : expr ,) => (assert_in ! ($ text , $ message)) ; ($ text : expr , $ message : expr , $ ($ arg : tt) +) => ({ match (&$ text , &$ message) { (text_val , message_val) => { if ! text_val . contains (message_val) { panic ! (r#"assertion failed: `text don't contain message`
         text: `{}`,
         message: `{}`: {}"# , text_val , message_val , format_args ! ($ ($ arg) +)) } } } }) ; }
};
}
