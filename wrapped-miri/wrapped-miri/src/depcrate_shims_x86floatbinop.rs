// Generated macro for FloatBinOp (enum)
macro_rules! Depcrate_shims_x86FloatBinOp {
() => {
// Module: crate::shims::x86
// Provides: {"FloatBinOp"}
// Dependencies: {}
# [derive (Copy , Clone)] enum FloatBinOp { # [doc = " Comparison"] # [doc = ""] # [doc = " The semantics of this operator is a case distinction: we compare the two operands,"] # [doc = " and then we return one of the four booleans `gt`, `lt`, `eq`, `unord` depending on"] # [doc = " which class they fall into."] # [doc = ""] # [doc = " AVX supports all 16 combinations, SSE only a subset"] # [doc = ""] # [doc = " <https://www.felixcloutier.com/x86/cmpss>"] # [doc = " <https://www.felixcloutier.com/x86/cmpps>"] # [doc = " <https://www.felixcloutier.com/x86/cmpsd>"] # [doc = " <https://www.felixcloutier.com/x86/cmppd>"] Cmp { # [doc = " Result when lhs < rhs"] gt : bool , # [doc = " Result when lhs > rhs"] lt : bool , # [doc = " Result when lhs == rhs"] eq : bool , # [doc = " Result when lhs is NaN or rhs is NaN"] unord : bool , } , # [doc = " Minimum value (with SSE semantics)"] # [doc = ""] # [doc = " <https://www.felixcloutier.com/x86/minss>"] # [doc = " <https://www.felixcloutier.com/x86/minps>"] # [doc = " <https://www.felixcloutier.com/x86/minsd>"] # [doc = " <https://www.felixcloutier.com/x86/minpd>"] Min , # [doc = " Maximum value (with SSE semantics)"] # [doc = ""] # [doc = " <https://www.felixcloutier.com/x86/maxss>"] # [doc = " <https://www.felixcloutier.com/x86/maxps>"] # [doc = " <https://www.felixcloutier.com/x86/maxsd>"] # [doc = " <https://www.felixcloutier.com/x86/maxpd>"] Max , }
};
}
