// Generated macro for impl_623 (impl)
macro_rules! Depcrate_expressionimpl_623 {
() => {
// Module: crate::expression
// Provides: {"impl_623"}
// Dependencies: {}
impl < T , DB > SelectableHelper < DB > for T where T : Selectable < DB > , DB : Backend , { fn as_select () -> AsSelect < Self , DB > { select_by :: SelectBy :: new () } }
};
}
