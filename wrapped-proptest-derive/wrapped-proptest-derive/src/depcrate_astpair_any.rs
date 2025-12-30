// Generated macro for pair_any (function)
macro_rules! Depcrate_astpair_any {
() => {
// Module: crate::ast
// Provides: {"pair_any"}
// Dependencies: {}
# [doc = " The type and constructor for `any::<Type>()`."] pub fn pair_any (ty : syn :: Type , span : Span) -> StratPair { let q = Ctor :: Arbitrary (ty . clone () , None , span) ; (Strategy :: Arbitrary (ty , span) , q) }
};
}
