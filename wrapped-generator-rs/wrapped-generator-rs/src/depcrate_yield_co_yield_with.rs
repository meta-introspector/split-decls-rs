// Generated macro for co_yield_with (function)
macro_rules! Depcrate_yield_co_yield_with {
() => {
// Module: crate::yield_
// Provides: {"co_yield_with"}
// Dependencies: {}
# [doc = " coroutine yield"] pub fn co_yield_with < T : Any > (v : T) { let env = ContextStack :: current () ; let context = env . co_ctx () . unwrap () ; if unlikely (context . _ref != 1) { std :: panic :: panic_any (Error :: Cancel) ; } context . co_set_ret (v) ; context . _ref -= 1 ; let parent = env . pop_context (context) ; let top = unsafe { & mut * context . parent } ; RegContext :: swap (& mut top . regs , & parent . regs) ; }
};
}
