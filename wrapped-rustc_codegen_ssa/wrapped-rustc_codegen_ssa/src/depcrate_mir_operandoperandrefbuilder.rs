// Generated macro for OperandRefBuilder (struct)
macro_rules! Depcrate_mir_operandOperandRefBuilder {
() => {
// Module: crate::mir::operand
// Provides: {"OperandRefBuilder"}
// Dependencies: {}
# [doc = " Allows building up an `OperandRef` by setting fields one at a time."] # [derive (Debug , Copy , Clone)] pub (super) struct OperandRefBuilder < 'tcx , V > { val : OperandValueBuilder < V > , layout : TyAndLayout < 'tcx > , }
};
}
