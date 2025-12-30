// Generated macro for assert_not_in (macro)
macro_rules! Depcrate_utilsassert_not_in {
() => {
// Module: crate::utils
// Provides: {"assert_not_in"}
// Dependencies: {}
# [macro_export] macro_rules ! assert_not_in { ($ text : expr , $ message : expr) => ({ match (&$ text , &$ message) { (text_val , message_val) => { if text_val . contains (message_val) { panic ! (r#"assertion failed: `text contains message`
         text: `{}`,
         message: `{}`"# , text_val , message_val) } } } }) ; ($ message : expr , $ expected : expr ,) => (assert_not_in ! ($ message , $ expected)) ; ($ text : expr , $ message : expr , $ ($ arg : tt) +) => ({ match (&$ text , &$ message) { (text_val , message_val) => { if text_val . contains (message_val) { panic ! (r#"assertion failed: `text contains message`
         text: `{}`,
         message: `{}`: {}"# , text_val , message_val , format_args ! ($ ($ arg) +)) } } } }) ; }
};
}
