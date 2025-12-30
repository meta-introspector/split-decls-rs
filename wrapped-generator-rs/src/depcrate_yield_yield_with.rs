// Generated macro for yield_with (function)
macro_rules! Depcrate_yield_yield_with {
() => {
// Module: crate::yield_
// Provides: {"yield_with"}
// Dependencies: {}
# [doc = " yield something without catch passed in para"] # [inline] # [deprecated (since = "0.6.18" , note = "please use `scope` version instead")] pub fn yield_with < T : Any > (v : T) { let env = ContextStack :: current () ; let context = env . top () ; raw_yield (& env , context , v) ; }
};
}
