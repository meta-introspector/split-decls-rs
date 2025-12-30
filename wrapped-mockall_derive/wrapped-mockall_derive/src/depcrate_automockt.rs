// Generated macro for t (module)
macro_rules! Depcrate_automockt {
() => {
// Module: crate::automock
// Provides: {"t"}
// Dependencies: {}
# [doc = " Unit tests for `Attrs`."] # [cfg (test)] mod t { use super :: super :: * ; use pretty_assertions :: assert_eq ; fn check_substitute_type (attrs : TokenStream , input : TokenStream , traitname : Ident , expected : TokenStream) { let _self : super :: Attrs = parse2 (attrs) . unwrap () ; let mut in_ty : Type = parse2 (input) . unwrap () ; let expect_ty : Type = parse2 (expected) . unwrap () ; _self . substitute_type (& mut in_ty , & traitname) ; assert_eq ! (in_ty , expect_ty) ; } # [test] fn qself () { check_substitute_type (quote ! (type T = u32 ;) , quote ! (< Self as Foo >:: T) , format_ident ! ("Foo") , quote ! (u32)) ; } # [test] # [should_panic (expected = "Mockall does not support QSelf substitutions except for the trait being mocked")] fn qself_other () { check_substitute_type (quote ! (type T = u32 ;) , quote ! (< Self as AsRef >:: T) , format_ident ! ("Foo") , quote ! (u32)) ; } # [test] # [should_panic (expected = "Unknown type substitution for QSelf")] fn unknown_substitution () { check_substitute_type (quote ! (type T = u32 ;) , quote ! (< Self as Foo >:: Q) , format_ident ! ("Foo") , quote ! (u32)) ; } }
};
}
