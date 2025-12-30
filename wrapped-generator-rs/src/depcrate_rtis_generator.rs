// Generated macro for is_generator (function)
macro_rules! Depcrate_rtis_generator {
() => {
// Module: crate::rt
// Provides: {"is_generator"}
// Dependencies: {}
# [doc = " check the current context if it's generator"] # [inline] pub fn is_generator () -> bool { let env = ContextStack :: current () ; let root = unsafe { & mut * env . root } ; ! root . child . is_null () }
};
}
