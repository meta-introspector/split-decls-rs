// Generated macro for impl_229 (impl)
macro_rules! Depcrate_builderimpl_229 {
() => {
// Module: crate::builder
// Provides: {"impl_229"}
// Dependencies: {}
impl ToGccComp for RealPredicate { fn to_gcc_comparison (& self) -> ComparisonOp { match * self { RealPredicate :: RealPredicateFalse => unreachable ! () , RealPredicate :: RealOEQ => ComparisonOp :: Equals , RealPredicate :: RealOGT => ComparisonOp :: GreaterThan , RealPredicate :: RealOGE => ComparisonOp :: GreaterThanEquals , RealPredicate :: RealOLT => ComparisonOp :: LessThan , RealPredicate :: RealOLE => ComparisonOp :: LessThanEquals , RealPredicate :: RealONE => ComparisonOp :: NotEquals , RealPredicate :: RealORD => unreachable ! () , RealPredicate :: RealUNO => unreachable ! () , RealPredicate :: RealUEQ => ComparisonOp :: Equals , RealPredicate :: RealUGT => ComparisonOp :: GreaterThan , RealPredicate :: RealUGE => ComparisonOp :: GreaterThan , RealPredicate :: RealULT => ComparisonOp :: LessThan , RealPredicate :: RealULE => ComparisonOp :: LessThan , RealPredicate :: RealUNE => ComparisonOp :: NotEquals , RealPredicate :: RealPredicateTrue => unreachable ! () , } } }
};
}
