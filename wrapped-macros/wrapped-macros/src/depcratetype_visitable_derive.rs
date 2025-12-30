// Generated macro for type_visitable_derive (function)
macro_rules! Depcratetype_visitable_derive {
() => {
// Module: crate
// Provides: {"type_visitable_derive"}
// Dependencies: {}
fn type_visitable_derive (mut s : synstructure :: Structure < '_ >) -> proc_macro2 :: TokenStream { if let syn :: Data :: Union (_) = s . ast () . data { panic ! ("cannot derive on union") } s . filter (| bi | { let mut ignored = false ; bi . ast () . attrs . iter () . for_each (| attr | { if ! attr . path () . is_ident ("type_visitable") { return ; } let _ = attr . parse_nested_meta (| nested | { if nested . path . is_ident ("ignore") { ignored = true ; } Ok (()) }) ; }) ; ! ignored }) ; if ! s . ast () . generics . lifetimes () . any (| lt | lt . lifetime . ident == "db") { s . add_impl_generic (parse_quote ! { 'db }) ; } s . add_bounds (synstructure :: AddBounds :: Generics) ; let body_visit = s . each (| bind | { quote ! { match :: rustc_type_ir :: VisitorResult :: branch (:: rustc_type_ir :: TypeVisitable :: visit_with (# bind , __visitor)) { :: core :: ops :: ControlFlow :: Continue (()) => { } , :: core :: ops :: ControlFlow :: Break (r) => { return :: rustc_type_ir :: VisitorResult :: from_residual (r) ; } , } } }) ; s . bind_with (| _ | synstructure :: BindStyle :: Move) ; s . bound_impl (quote ! (:: rustc_type_ir :: TypeVisitable <:: hir_ty :: next_solver :: DbInterner <'db >>) , quote ! { fn visit_with < __V : :: rustc_type_ir :: TypeVisitor <:: hir_ty :: next_solver :: DbInterner <'db >>> (& self , __visitor : & mut __V) -> __V :: Result { match * self { # body_visit } < __V :: Result as :: rustc_type_ir :: VisitorResult >:: output () } } ,) }
};
}
