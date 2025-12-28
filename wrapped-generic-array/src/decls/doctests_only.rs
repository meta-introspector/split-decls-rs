macro_rules! doctests_only {
    () => {
        mod doctests_only { # [doc = ""] # [doc = " Testing that lifetimes aren't transmuted when they're ellided."] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " #[macro_use] extern crate generic_array;"] # [doc = " fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {"] # [doc = "     arr![a as &A][0]"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ```rust"] # [doc = " #[macro_use] extern crate generic_array;"] # [doc = " fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'a A {"] # [doc = "     arr![a][0]"] # [doc = " }"] # [doc = " ```"] # [allow (dead_code)] pub enum DocTests { } }
    };
}

doctests_only!()