// Generated macro for macro_2755 (macro)
macro_rules! Depcrate_unfold_statemacro_2755 {
() => {
// Module: crate::unfold_state
// Provides: {"macro_2755"}
// Dependencies: {}
pin_project ! { # [doc = " UnfoldState used for stream and sink unfolds"] # [project = UnfoldStateProj] # [project_replace = UnfoldStateProjReplace] # [derive (Debug)] pub (crate) enum UnfoldState < T , Fut > { Value { value : T , } , Future { # [pin] future : Fut , } , Empty , } }
};
}
