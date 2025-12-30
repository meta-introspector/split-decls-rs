// Generated macro for format (macro)
macro_rules! Depcrate_collections_stringformat {
() => {
// Module: crate::collections::string
// Provides: {"format"}
// Dependencies: {}
# [doc = " Like the [`format!`] macro, but for creating [`bumpalo::collections::String`]s."] # [doc = ""] # [doc = " [`format!`]: https://doc.rust-lang.org/std/macro.format.html"] # [doc = " [`bumpalo::collections::String`]: collections/string/struct.String.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bumpalo::Bump;"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " let who = \"World\";"] # [doc = " let s = bumpalo::format!(in &b, \"Hello, {}!\", who);"] # [doc = " assert_eq!(s, \"Hello, World!\")"] # [doc = " ```"] # [macro_export] macro_rules ! format { (in $ bump : expr , $ fmt : expr , $ ($ args : expr) ,*) => { { use $ crate :: core_alloc :: fmt :: Write ; let bump = $ bump ; let mut s = $ crate :: collections :: String :: new_in (bump) ; let _ = write ! (& mut s , $ fmt , $ ($ args) ,*) ; s } } ; (in $ bump : expr , $ fmt : expr , $ ($ args : expr ,) *) => { $ crate :: format ! (in $ bump , $ fmt , $ ($ args) ,*) } ; }
};
}
