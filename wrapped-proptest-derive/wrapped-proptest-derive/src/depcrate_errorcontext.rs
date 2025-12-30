// Generated macro for Context (struct)
macro_rules! Depcrate_errorContext {
() => {
// Module: crate::error
// Provides: {"Context"}
// Dependencies: {}
# [doc = " The context / environment that the macro is operating in."] # [doc = " Right now, it simply tracks all the errors collected during"] # [doc = " the running of the macro."] # [derive (Default)] pub struct Context { errors : Vec < String > , }
};
}
