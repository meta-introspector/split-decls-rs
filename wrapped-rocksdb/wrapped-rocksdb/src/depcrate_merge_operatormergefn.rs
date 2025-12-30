// Generated macro for MergeFn (trait)
macro_rules! Depcrate_merge_operatorMergeFn {
() => {
// Module: crate::merge_operator
// Provides: {"MergeFn"}
// Dependencies: {}
pub trait MergeFn : Fn (& [u8] , Option < & [u8] > , & MergeOperands) -> Option < Vec < u8 > > + Send + Sync + 'static { }
};
}
