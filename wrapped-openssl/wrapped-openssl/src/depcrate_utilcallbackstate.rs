// Generated macro for CallbackState (struct)
macro_rules! Depcrate_utilCallbackState {
() => {
// Module: crate::util
// Provides: {"CallbackState"}
// Dependencies: {}
# [doc = " Wraps a user-supplied callback and a slot for panics thrown inside the callback (while FFI"] # [doc = " frames are on the stack)."] # [doc = ""] # [doc = " When dropped, checks if the callback has panicked, and resumes unwinding if so."] pub struct CallbackState < F > { # [doc = " The user callback. Taken out of the `Option` when called."] cb : Option < F > , # [doc = " If the callback panics, we place the panic object here, to be re-thrown once OpenSSL"] # [doc = " returns."] panic : Option < Box < dyn Any + Send + 'static > > , }
};
}
