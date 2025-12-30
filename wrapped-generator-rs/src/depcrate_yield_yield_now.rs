// Generated macro for yield_now (function)
macro_rules! Depcrate_yield_yield_now {
() => {
// Module: crate::yield_
// Provides: {"yield_now"}
// Dependencies: {}
# [doc = " switch back to parent context"] # [inline] pub fn yield_now () { let env = ContextStack :: current () ; let cur = env . top () ; raw_yield_now (& env , cur) ; }
};
}
