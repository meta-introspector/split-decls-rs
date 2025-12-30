// Generated macro for ErrorType (trait)
macro_rules! DepcrateErrorType {
() => {
// Module: crate
// Provides: {"ErrorType"}
// Dependencies: {}
# [doc = " Base trait for all IO traits, defining the error type."] # [doc = ""] # [doc = " All IO operations of all traits return the error defined in this trait."] # [doc = ""] # [doc = " Having a shared trait instead of having every trait define its own"] # [doc = " `Error` associated type enforces all impls on the same type use the same error."] # [doc = " This is very convenient when writing generic code, it means you have to"] # [doc = " handle a single error type `T::Error`, instead of `<T as Read>::Error` and `<T as Write>::Error`"] # [doc = " which might be different types."] pub trait ErrorType { # [doc = " Error type of all the IO operations on this type."] type Error : Error ; }
};
}
