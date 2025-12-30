// Generated macro for DynamicCoerceMany (type)
macro_rules! Depcrate_coercionDynamicCoerceMany {
() => {
// Module: crate::coercion
// Provides: {"DynamicCoerceMany"}
// Dependencies: {}
# [doc = " The type of a `CoerceMany` that is storing up the expressions into"] # [doc = " a buffer. We use this in `check/mod.rs` for things like `break`."] pub (crate) type DynamicCoerceMany < 'tcx > = CoerceMany < 'tcx , 'tcx , & 'tcx hir :: Expr < 'tcx > > ;
};
}
