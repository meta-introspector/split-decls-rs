// Generated macro for impl_1361 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1361 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1361"}
// Dependencies: {}
impl From < FloatCC > for FcmpImm { fn from (cond : FloatCC) -> Self { match cond { FloatCC :: Equal => FcmpImm :: Equal , FloatCC :: LessThan => FcmpImm :: LessThan , FloatCC :: LessThanOrEqual => FcmpImm :: LessThanOrEqual , FloatCC :: Unordered => FcmpImm :: Unordered , FloatCC :: NotEqual => FcmpImm :: NotEqual , FloatCC :: UnorderedOrGreaterThanOrEqual => FcmpImm :: UnorderedOrGreaterThanOrEqual , FloatCC :: UnorderedOrGreaterThan => FcmpImm :: UnorderedOrGreaterThan , FloatCC :: Ordered => FcmpImm :: Ordered , _ => panic ! ("unable to create comparison predicate for {cond}") , } } }
};
}
