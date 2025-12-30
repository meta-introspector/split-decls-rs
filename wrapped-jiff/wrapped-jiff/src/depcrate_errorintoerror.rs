// Generated macro for IntoError (trait)
macro_rules! Depcrate_errorIntoError {
() => {
// Module: crate::error
// Provides: {"IntoError"}
// Dependencies: {}
# [doc = " A simple trait to encapsulate automatic conversion to `Error`."] # [doc = ""] # [doc = " This trait basically exists to make `Error::context` work without needing"] # [doc = " to rely on public `From` impls. For example, without this trait, we might"] # [doc = " otherwise write `impl From<String> for Error`. But this would make it part"] # [doc = " of the public API. Which... maybe we should do, but at time of writing,"] # [doc = " I'm starting very conservative so that we can evolve errors in semver"] # [doc = " compatible ways."] pub (crate) trait IntoError { fn into_error (self) -> Error ; }
};
}
