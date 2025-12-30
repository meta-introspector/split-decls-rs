// Generated macro for once_err (function)
macro_rules! Depcrateonce_err {
() => {
// Module: crate
// Provides: {"once_err"}
// Dependencies: {}
# [doc = " Creates an iterator that fails with a predetermined error exactly once."] pub fn once_err < T , E > (value : E) -> OnceErr < T , E > { OnceErr (PhantomData , Some (value)) }
};
}
