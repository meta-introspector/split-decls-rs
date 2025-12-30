// Generated macro for writable_at_with_permissions (function)
macro_rules! Depcratewritable_at_with_permissions {
() => {
// Module: crate
// Provides: {"writable_at_with_permissions"}
// Dependencies: {}
# [doc = " Like [`writable_at`], but allows to set the given filesystem `permissions`."] pub fn writable_at_with_permissions (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove , permissions : std :: fs :: Permissions ,) -> io :: Result < Handle < Writable > > { Handle :: < Writable > :: at_with_permissions (path , directory , cleanup , permissions) }
};
}
