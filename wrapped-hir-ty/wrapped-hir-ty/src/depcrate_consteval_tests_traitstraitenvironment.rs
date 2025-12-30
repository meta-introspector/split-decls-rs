// Generated macro for TraitEnvironment (struct)
macro_rules! Depcrate_consteval_tests_traitsTraitEnvironment {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"TraitEnvironment"}
// Dependencies: {}
# [doc = " A set of clauses that we assume to be true. E.g. if we are inside this function:"] # [doc = " ```rust"] # [doc = " fn foo<T: Default>(t: T) {}"] # [doc = " ```"] # [doc = " we assume that `T: Default`."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct TraitEnvironment < 'db > { pub krate : Crate , pub block : Option < BlockId > , traits_from_clauses : Box < [(Ty < 'db > , TraitId)] > , pub env : ParamEnv < 'db > , }
};
}
