// Generated macro for OwnedStore (struct)
macro_rules! Depcrate_bridge_handleOwnedStore {
() => {
// Module: crate::bridge::handle
// Provides: {"OwnedStore"}
// Dependencies: {}
# [doc = " A store that associates values of type `T` with numeric handles. A value can"] # [doc = " be looked up using its handle."] pub (super) struct OwnedStore < T : 'static > { counter : & 'static AtomicU32 , data : BTreeMap < Handle , T > , }
};
}
