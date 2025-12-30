// Generated macro for AsyncFnVisitor (struct)
macro_rules! Depcrate_unused_asyncAsyncFnVisitor {
() => {
// Module: crate::unused_async
// Provides: {"AsyncFnVisitor"}
// Dependencies: {}
struct AsyncFnVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , found_await : bool , # [doc = " Also keep track of `await`s in nested async blocks so we can mention"] # [doc = " it in a note"] await_in_async_block : Option < Span > , async_depth : usize , }
};
}
