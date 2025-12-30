// Generated macro for Display (struct)
macro_rules! Depcrate_attrDisplay {
() => {
// Module: crate::attr
// Provides: {"Display"}
// Dependencies: {}
# [derive (Clone)] pub struct Display < 'a > { pub original : & 'a Attribute , pub fmt : LitStr , pub args : TokenStream , pub requires_fmt_machinery : bool , pub has_bonus_display : bool , pub infinite_recursive : bool , pub implied_bounds : Set < (usize , Trait) > , pub bindings : Vec < (Ident , TokenStream) > , }
};
}
