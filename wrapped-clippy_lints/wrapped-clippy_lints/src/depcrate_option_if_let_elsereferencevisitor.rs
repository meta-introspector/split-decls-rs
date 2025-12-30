// Generated macro for ReferenceVisitor (struct)
macro_rules! Depcrate_option_if_let_elseReferenceVisitor {
() => {
// Module: crate::option_if_let_else
// Provides: {"ReferenceVisitor"}
// Dependencies: {}
# [doc = " This visitor checks if the <none> block contains references to the local variables that are"] # [doc = " used in the <then> block. See [`ConditionVisitor`] for more."] struct ReferenceVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , identifiers : FxHashSet < HirId > , }
};
}
