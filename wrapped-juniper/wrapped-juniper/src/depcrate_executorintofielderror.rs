// Generated macro for IntoFieldError (trait)
macro_rules! Depcrate_executorIntoFieldError {
() => {
// Module: crate::executor
// Provides: {"IntoFieldError"}
// Dependencies: {}
# [doc = " Custom error handling trait to enable error types other than [`FieldError`]"] # [doc = " to be specified as return value."] # [doc = ""] # [doc = " Any custom error type should implement this trait to convert itself into a"] # [doc = " [`FieldError`]."] pub trait IntoFieldError < S = DefaultScalarValue > { # [doc = " Performs the custom conversion into a [`FieldError`]."] # [must_use] fn into_field_error (self) -> FieldError < S > ; }
};
}
