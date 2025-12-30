// Generated macro for InternedStore (struct)
macro_rules! Depcrate_bridge_handleInternedStore {
() => {
// Module: crate::bridge::handle
// Provides: {"InternedStore"}
// Dependencies: {}
# [doc = " Like `OwnedStore`, but avoids storing any value more than once."] pub (super) struct InternedStore < T : 'static > { owned : OwnedStore < T > , interner : FxHashMap < T , Handle > , }
};
}
