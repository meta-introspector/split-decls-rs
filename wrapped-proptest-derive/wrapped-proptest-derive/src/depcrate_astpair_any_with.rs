// Generated macro for pair_any_with (function)
macro_rules! Depcrate_astpair_any_with {
() => {
// Module: crate::ast
// Provides: {"pair_any_with"}
// Dependencies: {}
# [doc = " The type and constructor for `any_with::<Type>(parameters)`."] pub fn pair_any_with (ty : syn :: Type , var : usize , span : Span) -> StratPair { let q = Ctor :: Arbitrary (ty . clone () , Some (var) , span) ; (Strategy :: Arbitrary (ty , span) , q) }
};
}
