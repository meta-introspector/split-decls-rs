// Generated macro for __enum_keyword_inner (macro)
macro_rules! Depcrate_preferences_extensions_unicode_macros_enum_keyword__enum_keyword_inner {
() => {
// Module: crate::preferences::extensions::unicode::macros::enum_keyword
// Provides: {"__enum_keyword_inner"}
// Dependencies: {}
# [doc = " Internal macro used by `enum_keyword` for nesting."] # [macro_export] # [doc (hidden)] macro_rules ! __enum_keyword_inner { ($ name : ident , $ variant : ident) => { $ name ::$ variant } ; ($ name : ident , $ variant : ident , $ s : ident , $ v2 : ident , $ ($ subk : expr => $ subv : ident) ,*) => { { let sv = $ s . get_subtag (1) . and_then (| st | { match st . as_str () { $ ($ subk => Some ($ v2 ::$ subv) ,) * _ => None , } }) ; $ name ::$ variant (sv) } } ; }
};
}
