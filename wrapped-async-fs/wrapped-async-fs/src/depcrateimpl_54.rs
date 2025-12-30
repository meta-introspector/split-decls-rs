// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
# [cfg (windows)] impl From < std :: os :: windows :: io :: OwnedHandle > for File { fn from (fd : std :: os :: windows :: io :: OwnedHandle) -> Self { File :: from (std :: fs :: File :: from (fd)) } }
};
}
