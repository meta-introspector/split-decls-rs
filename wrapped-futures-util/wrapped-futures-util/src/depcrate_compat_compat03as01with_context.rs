// Generated macro for with_context (function)
macro_rules! Depcrate_compat_compat03as01with_context {
() => {
// Module: crate::compat::compat03as01
// Provides: {"with_context"}
// Dependencies: {}
fn with_context < T , R , F > (compat : & mut Compat < T > , f : F) -> R where T : Unpin , F : FnOnce (Pin < & mut T > , & mut Context < '_ >) -> R , { let current = Current :: new () ; let waker = current . as_waker () ; let mut cx = Context :: from_waker (& waker) ; f (Pin :: new (& mut compat . inner) , & mut cx) }
};
}
