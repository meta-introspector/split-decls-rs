// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
# [cfg (unix)] impl From < std :: os :: unix :: io :: OwnedFd > for File { fn from (fd : std :: os :: unix :: io :: OwnedFd) -> Self { File :: from (std :: fs :: File :: from (fd)) } }
};
}
