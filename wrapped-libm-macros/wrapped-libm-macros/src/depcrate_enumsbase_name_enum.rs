// Generated macro for base_name_enum (function)
macro_rules! Depcrate_enumsbase_name_enum {
() => {
// Module: crate::enums
// Provides: {"base_name_enum"}
// Dependencies: {}
# [doc = " Implement `#[base_name_enum]`, see documentation in `lib.rs`."] pub fn base_name_enum (mut item : ItemEnum , attributes : pm2 :: TokenStream ,) -> syn :: Result < pm2 :: TokenStream > { expect_empty_enum (& item) ? ; if ! attributes . is_empty () { let sp = attributes . span () ; return Err (syn :: Error :: new (sp . span () , "no attributes expected")) ; } let mut base_names : Vec < _ > = ALL_OPERATIONS . iter () . map (| func | base_name (func . name)) . collect () ; base_names . sort_unstable () ; base_names . dedup () ; let item_name = & item . ident ; let mut as_str_arms = Vec :: new () ; for base_name in base_names { let ident = Ident :: new (& base_name . to_upper_camel_case () , Span :: call_site ()) ; as_str_arms . push (quote ! { Self ::# ident => # base_name }) ; let variant = Variant { attrs : Vec :: new () , ident , fields : Fields :: Unit , discriminant : None , } ; item . variants . push (variant) ; } let res = quote ! { # item impl # item_name { # [doc = " The stringified version of this base name."] pub const fn as_str (self) -> &'static str { match self { # (# as_str_arms) ,* } } } } ; Ok (res) }
};
}
