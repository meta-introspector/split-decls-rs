// Generated macro for lower_fp_condcode (function)
macro_rules! Depcrate_isa_aarch64_lowerlower_fp_condcode {
() => {
// Module: crate::isa::aarch64::lower
// Provides: {"lower_fp_condcode"}
// Dependencies: {}
pub (crate) fn lower_fp_condcode (cc : FloatCC) -> Cond { match cc { FloatCC :: Ordered => Cond :: Vc , FloatCC :: Unordered => Cond :: Vs , FloatCC :: Equal => Cond :: Eq , FloatCC :: NotEqual => Cond :: Ne , FloatCC :: OrderedNotEqual => unimplemented ! () , FloatCC :: UnorderedOrEqual => unimplemented ! () , FloatCC :: LessThan => Cond :: Mi , FloatCC :: LessThanOrEqual => Cond :: Ls , FloatCC :: GreaterThan => Cond :: Gt , FloatCC :: GreaterThanOrEqual => Cond :: Ge , FloatCC :: UnorderedOrLessThan => unimplemented ! () , FloatCC :: UnorderedOrLessThanOrEqual => unimplemented ! () , FloatCC :: UnorderedOrGreaterThan => unimplemented ! () , FloatCC :: UnorderedOrGreaterThanOrEqual => unimplemented ! () , } }
};
}
