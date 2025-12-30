// Generated macro for tests (module)
macro_rules! Depcrate_usage_lifetimestests {
() => {
// Module: crate::usage::lifetimes
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use proc_macro2 :: Span ; use syn :: { parse_quote , DeriveInput } ; use super :: UsesLifetimes ; use crate :: usage :: GenericsExt ; use crate :: usage :: Purpose :: * ; # [test] fn struct_named () { let input : DeriveInput = parse_quote ! { struct Foo <'a , 'b : 'a > { parent : &'b Bar , child : &'a Baz , } } ; let omitted = syn :: Lifetime :: new ("'c" , Span :: call_site ()) ; let lifetimes = { let mut lt = input . generics . declared_lifetimes () ; lt . insert (omitted) ; lt } ; let matches = input . data . uses_lifetimes (& BoundImpl . into () , & lifetimes) ; assert_eq ! (matches . len () , 2) ; } # [test] fn qself () { let input : DeriveInput = parse_quote ! { struct Foo <'a , 'b : 'a > { parent : &'b Bar , child : < Bar <'a > as MyIterator >:: Item , } } ; let lifetimes = input . generics . declared_lifetimes () ; let matches = input . data . uses_lifetimes (& BoundImpl . into () , & lifetimes) ; assert_eq ! (matches . len () , 1) ; let decl_matches = input . data . uses_lifetimes (& Declare . into () , & lifetimes) ; assert_eq ! (decl_matches . len () , 2) ; } }
};
}
