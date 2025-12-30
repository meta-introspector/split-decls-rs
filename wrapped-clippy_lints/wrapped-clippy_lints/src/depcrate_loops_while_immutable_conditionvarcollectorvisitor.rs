// Generated macro for VarCollectorVisitor (struct)
macro_rules! Depcrate_loops_while_immutable_conditionVarCollectorVisitor {
() => {
// Module: crate::loops::while_immutable_condition
// Provides: {"VarCollectorVisitor"}
// Dependencies: {}
# [doc = " Collects the set of variables in an expression"] # [doc = " Stops analysis if a function call is found"] # [doc = " Note: In some cases such as `self`, there are no mutable annotation,"] # [doc = " All variables definition IDs are collected"] struct VarCollectorVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , ids : HirIdSet , def_ids : DefIdMap < bool > , }
};
}
