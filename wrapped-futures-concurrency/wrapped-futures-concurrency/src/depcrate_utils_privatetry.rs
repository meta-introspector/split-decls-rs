// Generated macro for Try (trait)
macro_rules! Depcrate_utils_privateTry {
() => {
// Module: crate::utils::private
// Provides: {"Try"}
// Dependencies: {}
# [doc = " Clone of `std::ops::Try`."] # [doc = ""] # [doc = " Implementing this trait is not permitted outside of `futures_concurrency`."] pub trait Try { private_decl ! { } type Output ; type Residual ; fn from_output (output : Self :: Output) -> Self ; fn from_residual (residual : Self :: Residual) -> Self ; fn branch (self) -> ControlFlow < Self :: Residual , Self :: Output > ; }
};
}
