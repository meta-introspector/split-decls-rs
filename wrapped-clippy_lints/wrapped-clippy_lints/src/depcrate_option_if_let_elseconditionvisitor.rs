// Generated macro for ConditionVisitor (struct)
macro_rules! Depcrate_option_if_let_elseConditionVisitor {
() => {
// Module: crate::option_if_let_else
// Provides: {"ConditionVisitor"}
// Dependencies: {}
# [doc = " This visitor looks for bindings in the <then> block that mention a local variable. Then gets the"] # [doc = " identifiers. The list of identifiers will then be used to check if the <none> block mentions the"] # [doc = " same local. See [`ReferenceVisitor`] for more."] struct ConditionVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , identifiers : FxHashSet < HirId > , }
};
}
