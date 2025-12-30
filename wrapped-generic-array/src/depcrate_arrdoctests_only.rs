// Generated macro for doctests_only (module)
macro_rules! Depcrate_arrdoctests_only {
() => {
// Module: crate::arr
// Provides: {"doctests_only"}
// Dependencies: {}
mod doctests_only { # [doc = ""] # [doc = " Testing that lifetimes aren't transmuted when they're ellided."] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " #[macro_use] extern crate generic_array;"] # [doc = " fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {"] # [doc = "     arr![a as &A][0]"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ```rust"] # [doc = " #[macro_use] extern crate generic_array;"] # [doc = " fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'a A {"] # [doc = "     arr![a][0]"] # [doc = " }"] # [doc = " ```"] # [allow (dead_code)] pub enum DocTests { } }
};
}
