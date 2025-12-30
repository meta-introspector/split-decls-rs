// Generated macro for Error (trait)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error trait."] # [doc = ""] # [doc = " This trait allows generic code to do limited inspecting of errors,"] # [doc = " to react differently to different kinds."] pub trait Error : core :: error :: Error { # [doc = " Get the kind of this error."] fn kind (& self) -> ErrorKind ; }
};
}
