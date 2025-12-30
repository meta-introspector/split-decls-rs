// Generated macro for mark_at_with_permissions (function)
macro_rules! Depcratemark_at_with_permissions {
() => {
// Module: crate
// Provides: {"mark_at_with_permissions"}
// Dependencies: {}
# [doc = " Like [`mark_at`], but allows to set the given filesystem `permissions`."] pub fn mark_at_with_permissions (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove , permissions : std :: fs :: Permissions ,) -> io :: Result < Handle < Closed > > { Handle :: < Closed > :: at_with_permissions (path , directory , cleanup , permissions) }
};
}
