// Generated macro for DynSync (trait)
macro_rules! Depcrate_markerDynSync {
() => {
// Module: crate::marker
// Provides: {"DynSync"}
// Dependencies: {}
# [diagnostic :: on_unimplemented (message = "`{Self}` doesn't implement `DynSync`. \
            Add it to `rustc_data_structures::marker` or use `IntoDynSyncSend` if it's already `Sync`")] pub unsafe auto trait DynSync { }
};
}
