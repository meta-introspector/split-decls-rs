// Generated macro for UndoLog (enum)
macro_rules! Depcrate_infer_type_variableUndoLog {
() => {
// Module: crate::infer::type_variable
// Provides: {"UndoLog"}
// Dependencies: {}
# [doc = " Represents a single undo-able action that affects a type inference variable."] # [derive (Clone)] pub (crate) enum UndoLog < 'tcx > { EqRelation (sv :: UndoLog < ut :: Delegate < TyVidEqKey < 'tcx > > >) , SubRelation (sv :: UndoLog < ut :: Delegate < TyVidSubKey > >) , }
};
}
