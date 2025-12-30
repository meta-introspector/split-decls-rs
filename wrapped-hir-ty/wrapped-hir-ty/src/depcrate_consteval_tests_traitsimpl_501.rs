// Generated macro for impl_501 (impl)
macro_rules! Depcrate_consteval_tests_traitsimpl_501 {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"impl_501"}
// Dependencies: {}
impl TraitEnvironment { pub fn empty (krate : Crate) -> Arc < Self > { Arc :: new (TraitEnvironment { krate , block : None , traits_from_clauses : Box :: default () , env : chalk_ir :: Environment :: new (Interner) , }) } pub fn new (krate : Crate , block : Option < BlockId > , traits_from_clauses : Box < [(Ty , TraitId)] > , env : chalk_ir :: Environment < Interner > ,) -> Arc < Self > { Arc :: new (TraitEnvironment { krate , block , traits_from_clauses , env }) } pub fn with_block (this : & mut Arc < Self > , block : BlockId) { Arc :: make_mut (this) . block = Some (block) ; } pub fn traits_in_scope_from_clauses (& self , ty : Ty) -> impl Iterator < Item = TraitId > + '_ { self . traits_from_clauses . iter () . filter_map (move | (self_ty , trait_id) | (* self_ty == ty) . then_some (* trait_id)) } }
};
}
