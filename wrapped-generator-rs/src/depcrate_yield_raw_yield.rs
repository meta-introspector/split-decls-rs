// Generated macro for raw_yield (function)
macro_rules! Depcrate_yield_raw_yield {
() => {
// Module: crate::yield_
// Provides: {"raw_yield"}
// Dependencies: {}
# [doc = " raw yield without catch passed in para"] # [inline] fn raw_yield < T : Any > (env : & ContextStack , context : & mut Context , v : T) { if unlikely (! context . is_generator ()) { panic ! ("yield from none generator context") ; } context . set_ret (v) ; context . _ref -= 1 ; raw_yield_now (env , context) ; if unlikely (context . _ref != 1) { std :: panic :: panic_any (Error :: Cancel) ; } }
};
}
