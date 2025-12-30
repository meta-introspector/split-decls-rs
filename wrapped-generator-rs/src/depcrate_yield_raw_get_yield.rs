// Generated macro for raw_get_yield (function)
macro_rules! Depcrate_yield_raw_get_yield {
() => {
// Module: crate::yield_
// Provides: {"raw_get_yield"}
// Dependencies: {}
# [doc = " get the passed in para from context"] # [inline] fn raw_get_yield < A : Any > (context : & mut Context) -> Option < A > { if unlikely (! context . is_generator ()) { { error ! ("get yield from none generator context") ; std :: panic :: panic_any (Error :: ContextErr) ; } } context . get_para () }
};
}
