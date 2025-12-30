// Generated macro for ParameterError (struct)
macro_rules! Depcrate_errorParameterError {
() => {
// Module: crate::error
// Provides: {"ParameterError"}
// Dependencies: {}
# [doc = " An error was encountered in inputs arguments."] # [doc = ""] # [doc = " This is used as an opaque representation for the [`ImageError::Parameter`] variant. See its"] # [doc = " documentation for more information."] # [doc = ""] # [doc = " [`ImageError::Parameter`]: enum.ImageError.html#variant.Parameter"] # [derive (Debug)] pub struct ParameterError { kind : ParameterErrorKind , underlying : Option < Box < dyn Error + Send + Sync > > , }
};
}
