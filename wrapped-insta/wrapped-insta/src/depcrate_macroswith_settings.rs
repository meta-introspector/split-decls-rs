// Generated macro for with_settings (macro)
macro_rules! Depcrate_macroswith_settings {
() => {
// Module: crate::macros
// Provides: {"with_settings"}
// Dependencies: {}
# [doc = " Settings configuration macro."] # [doc = ""] # [doc = " This macro lets you bind some [`Settings`](crate::Settings) temporarily.  The first argument"] # [doc = " takes key value pairs that should be set, and the second is the block to"] # [doc = " execute.  All settings can be set (`sort_maps => value` maps to `set_sort_maps(value)`)."] # [doc = " The exception are redactions, which can only be set to a vector this way."] # [doc = ""] # [doc = " This example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " insta::with_settings!({sort_maps => true}, {"] # [doc = "     // run snapshot test here"] # [doc = " });"] # [doc = " ```"] # [doc = ""] # [doc = " Is equivalent to the following:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use insta::Settings;"] # [doc = " let mut settings = Settings::clone_current();"] # [doc = " settings.set_sort_maps(true);"] # [doc = " settings.bind(|| {"] # [doc = "     // run snapshot test here"] # [doc = " });"] # [doc = " ```"] # [doc = ""] # [doc = " Note: before insta 0.17, this macro used"] # [doc = " [`Settings::new`](crate::Settings::new) which meant that original settings"] # [doc = " were always reset rather than extended."] # [macro_export] macro_rules ! with_settings { ({ $ ($ k : ident => $ v : expr) ,*$ (,) ? } , $ body : block) => { { let mut settings = $ crate :: Settings :: clone_current () ; $ (settings . _private_inner_mut () .$ k ($ v) ;) * settings . bind (|| $ body) } } }
};
}
