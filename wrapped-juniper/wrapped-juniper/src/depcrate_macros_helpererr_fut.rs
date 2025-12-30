// Generated macro for err_fut (function)
macro_rules! Depcrate_macros_helpererr_fut {
() => {
// Module: crate::macros::helper
// Provides: {"err_fut"}
// Dependencies: {}
# [doc = " Wraps `msg` with [`Display`] implementation into opaque [`Send`] [`Future`]"] # [doc = " which immediately resolves into [`FieldError`]."] pub fn err_fut < 'ok , D , Ok , S > (msg : D) -> BoxFuture < 'ok , Result < Ok , FieldError < S > > > where D : Display , Ok : Send + 'ok , S : Send + 'static , { Box :: pin (future :: err (FieldError :: from (msg))) }
};
}
