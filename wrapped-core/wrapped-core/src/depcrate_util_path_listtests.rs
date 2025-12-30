// Generated macro for tests (module)
macro_rules! Depcrate_util_path_listtests {
() => {
// Module: crate::util::path_list
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: PathList ; use crate :: FromMeta ; use proc_macro2 :: TokenStream ; use quote :: quote ; use syn :: { parse_quote , Attribute , Meta } ; # [doc = " parse a string as a syn::Meta instance."] fn pm (tokens : TokenStream) -> :: std :: result :: Result < Meta , String > { let attribute : Attribute = parse_quote ! (# [# tokens]) ; Ok (attribute . meta) } fn fm < T : FromMeta > (tokens : TokenStream) -> T { FromMeta :: from_meta (& pm (tokens) . expect ("Tests should pass well-formed input")) . expect ("Tests should pass valid input") } # [test] fn succeeds () { let paths = fm :: < PathList > (quote ! (ignore (Debug , Clone , Eq))) ; assert_eq ! (paths . to_strings () , vec ! [String :: from ("Debug") , String :: from ("Clone") , String :: from ("Eq")]) ; } # [doc = " Check that the parser rejects non-word members of the list, and that the error"] # [doc = " has an associated span."] # [test] fn fails_non_word () { let input = PathList :: from_meta (& pm (quote ! (ignore (Debug , Clone = false))) . unwrap ()) ; let err = input . unwrap_err () ; assert ! (err . has_span ()) ; } # [test] fn intersection () { let left = fm :: < PathList > (quote ! (ignore (Debug , Clone , Eq))) ; let right = fm :: < PathList > (quote ! (ignore (Clone , Eq , Clone))) ; assert_eq ! (left . intersection (& right) . cloned () . collect ::< Vec < _ >> () , vec ! [parse_quote ! (Clone) , parse_quote ! (Eq)] ,) ; } }
};
}
