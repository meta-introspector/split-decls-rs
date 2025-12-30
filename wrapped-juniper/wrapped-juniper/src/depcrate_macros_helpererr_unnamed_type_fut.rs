// Generated macro for err_unnamed_type_fut (function)
macro_rules! Depcrate_macros_helpererr_unnamed_type_fut {
() => {
// Module: crate::macros::helper
// Provides: {"err_unnamed_type_fut"}
// Dependencies: {}
# [doc = " Returns a [`future::err`] wrapping the [`err_unnamed_type`]."] pub fn err_unnamed_type_fut < 'ok , Ok , S > (name : & str) -> BoxFuture < 'ok , Result < Ok , FieldError < S > > > where Ok : Send + 'ok , S : Send + 'static , { Box :: pin (future :: err (err_unnamed_type (name))) }
};
}
