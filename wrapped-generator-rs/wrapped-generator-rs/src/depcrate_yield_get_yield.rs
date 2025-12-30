// Generated macro for get_yield (function)
macro_rules! Depcrate_yield_get_yield {
() => {
// Module: crate::yield_
// Provides: {"get_yield"}
// Dependencies: {}
# [doc = " get the passed in para"] # [inline] # [deprecated (since = "0.6.18" , note = "please use `scope` version instead")] pub fn get_yield < A : Any > () -> Option < A > { let context = ContextStack :: current () . top () ; raw_get_yield (context) }
};
}
