// Generated macro for EagerCallInfo (struct)
macro_rules! DepcrateEagerCallInfo {
() => {
// Module: crate
// Provides: {"EagerCallInfo"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct EagerCallInfo { # [doc = " The expanded argument of the eager macro."] arg : Arc < tt :: TopSubtree > , # [doc = " Call id of the eager macro's input file (this is the macro file for its fully expanded input)."] arg_id : MacroCallId , error : Option < ExpandError > , # [doc = " The call site span of the eager macro"] span : Span , }
};
}
