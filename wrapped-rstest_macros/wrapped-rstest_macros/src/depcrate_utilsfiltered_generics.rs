// Generated macro for filtered_generics (function)
macro_rules! Depcrate_utilsfiltered_generics {
() => {
// Module: crate::utils
// Provides: {"filtered_generics"}
// Dependencies: {}
fn filtered_generics < 'a > (params : impl Iterator < Item = syn :: GenericParam > + 'a , valids : & 'a HashSet < Ident > ,) -> impl Iterator < Item = syn :: GenericParam > + 'a { params . filter (move | p | match p . maybe_ident () { Some (id) => valids . contains (id) , None => false , }) }
};
}
