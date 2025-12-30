// Generated macro for yield_ (function)
macro_rules! Depcrate_yield_yield_ {
() => {
// Module: crate::yield_
// Provides: {"yield_"}
// Dependencies: {}
# [doc = " yield and get the send para"] # [inline] # [deprecated (since = "0.6.18" , note = "please use `scope` version instead")] pub fn yield_ < A : Any , T : Any > (v : T) -> Option < A > { let env = ContextStack :: current () ; let context = env . top () ; raw_yield (& env , context , v) ; atomic :: compiler_fence (atomic :: Ordering :: Acquire) ; raw_get_yield (context) }
};
}
