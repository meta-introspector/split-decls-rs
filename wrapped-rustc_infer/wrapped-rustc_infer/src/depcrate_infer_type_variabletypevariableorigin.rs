// Generated macro for TypeVariableOrigin (struct)
macro_rules! Depcrate_infer_type_variableTypeVariableOrigin {
() => {
// Module: crate::infer::type_variable
// Provides: {"TypeVariableOrigin"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub struct TypeVariableOrigin { pub span : Span , # [doc = " `DefId` of the type parameter this was instantiated for, if any."] # [doc = ""] # [doc = " This should only be used for diagnostics."] pub param_def_id : Option < DefId > , }
};
}
