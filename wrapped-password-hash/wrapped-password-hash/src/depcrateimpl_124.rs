// Generated macro for impl_124 (impl)
macro_rules! Depcrateimpl_124 {
() => {
// Module: crate
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl From < & PasswordHash < '_ > > for PasswordHashString { fn from (hash : & PasswordHash < '_ >) -> PasswordHashString { PasswordHashString { string : hash . to_string () , encoding : hash . encoding () , } } }
};
}
