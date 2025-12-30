// Generated macro for init_and_set_handler (function)
macro_rules! Depcrateinit_and_set_handler {
() => {
// Module: crate
// Provides: {"init_and_set_handler"}
// Dependencies: {}
fn init_and_set_handler < F > (user_handler : F , overwrite : bool) -> Result < () , Error > where F : FnMut () + 'static + Send , { if ! INIT . load (Ordering :: Acquire) { let _guard = INIT_LOCK . lock () . unwrap () ; if ! INIT . load (Ordering :: Relaxed) { set_handler_inner (user_handler , overwrite) ? ; INIT . store (true , Ordering :: Release) ; return Ok (()) ; } } Err (Error :: MultipleHandlers) }
};
}
