// Generated macro for HandleError (enum)
macro_rules! Depcrate_shims_windows_handleHandleError {
() => {
// Module: crate::shims::windows::handle
// Provides: {"HandleError"}
// Dependencies: {}
# [doc = " Errors that can occur when constructing a [`Handle`] from a Scalar."] pub enum HandleError { # [doc = " There is no thread with the given ID."] ThreadNotFound (ThreadNotFound) , # [doc = " Can't convert scalar to handle because it is structurally invalid."] InvalidHandle , }
};
}
