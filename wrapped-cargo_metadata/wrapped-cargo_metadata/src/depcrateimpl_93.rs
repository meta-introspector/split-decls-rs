// Generated macro for impl_93 (impl)
macro_rules! Depcrateimpl_93 {
() => {
// Module: crate
// Provides: {"impl_93"}
// Dependencies: {}
impl core :: ops :: Deref for WorkspaceDefaultMembers { type Target = [PackageId] ; fn deref (& self) -> & Self :: Target { self . 0 . as_ref () . expect ("WorkspaceDefaultMembers should only be dereferenced on Cargo versions >= 1.71") } }
};
}
