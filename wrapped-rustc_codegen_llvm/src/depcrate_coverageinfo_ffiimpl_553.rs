// Generated macro for impl_553 (impl)
macro_rules! Depcrate_coverageinfo_ffiimpl_553 {
() => {
// Module: crate::coverageinfo::ffi
// Provides: {"impl_553"}
// Dependencies: {}
impl Counter { # [doc = " A `Counter` of kind `Zero`. For this counter kind, the `id` is not used."] pub (crate) const ZERO : Self = Self { kind : CounterKind :: Zero , id : 0 } ; # [doc = " Constructs a new `Counter` of kind `CounterValueReference`."] pub (crate) fn counter_value_reference (counter_id : CounterId) -> Self { Self { kind : CounterKind :: CounterValueReference , id : counter_id . as_u32 () } } # [doc = " Constructs a new `Counter` of kind `Expression`."] pub (crate) fn expression (expression_id : ExpressionId) -> Self { Self { kind : CounterKind :: Expression , id : expression_id . as_u32 () } } pub (crate) fn from_term (term : CovTerm) -> Self { match term { CovTerm :: Zero => Self :: ZERO , CovTerm :: Counter (id) => Self :: counter_value_reference (id) , CovTerm :: Expression (id) => Self :: expression (id) , } } }
};
}
