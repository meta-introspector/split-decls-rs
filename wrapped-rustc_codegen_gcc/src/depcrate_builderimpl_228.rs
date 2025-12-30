// Generated macro for impl_228 (impl)
macro_rules! Depcrate_builderimpl_228 {
() => {
// Module: crate::builder
// Provides: {"impl_228"}
// Dependencies: {}
impl ToGccComp for IntPredicate { fn to_gcc_comparison (& self) -> ComparisonOp { match * self { IntPredicate :: IntEQ => ComparisonOp :: Equals , IntPredicate :: IntNE => ComparisonOp :: NotEquals , IntPredicate :: IntUGT => ComparisonOp :: GreaterThan , IntPredicate :: IntUGE => ComparisonOp :: GreaterThanEquals , IntPredicate :: IntULT => ComparisonOp :: LessThan , IntPredicate :: IntULE => ComparisonOp :: LessThanEquals , IntPredicate :: IntSGT => ComparisonOp :: GreaterThan , IntPredicate :: IntSGE => ComparisonOp :: GreaterThanEquals , IntPredicate :: IntSLT => ComparisonOp :: LessThan , IntPredicate :: IntSLE => ComparisonOp :: LessThanEquals , } } }
};
}
