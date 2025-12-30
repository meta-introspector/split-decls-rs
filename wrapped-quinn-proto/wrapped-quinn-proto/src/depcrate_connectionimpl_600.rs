// Generated macro for impl_600 (impl)
macro_rules! Depcrate_connectionimpl_600 {
() => {
// Module: crate::connection
// Provides: {"impl_600"}
// Dependencies: {}
impl From < Close > for ConnectionError { fn from (x : Close) -> Self { match x { Close :: Connection (reason) => Self :: ConnectionClosed (reason) , Close :: Application (reason) => Self :: ApplicationClosed (reason) , } } }
};
}
