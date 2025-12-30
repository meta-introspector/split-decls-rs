// Generated macro for impl_128 (impl)
macro_rules! Depcrateimpl_128 {
() => {
// Module: crate
// Provides: {"impl_128"}
// Dependencies: {}
impl From < WeightedRelation > for (RelationalOperator , f64) { fn from (r : WeightedRelation) -> (RelationalOperator , f64) { use WeightedRelation :: * ; match r { EQ (s) => (RelationalOperator :: Equal , s) , LE (s) => (RelationalOperator :: LessOrEqual , s) , GE (s) => (RelationalOperator :: GreaterOrEqual , s) , } } }
};
}
