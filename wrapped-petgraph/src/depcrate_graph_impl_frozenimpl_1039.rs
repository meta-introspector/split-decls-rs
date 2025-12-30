// Generated macro for impl_1039 (impl)
macro_rules! Depcrate_graph_impl_frozenimpl_1039 {
() => {
// Module: crate::graph_impl::frozen
// Provides: {"impl_1039"}
// Dependencies: {}
# [doc = " Deref allows transparent access to all shared reference (read-only)"] # [doc = " functionality in the underlying graph."] impl < G > Deref for Frozen < '_ , G > { type Target = G ; fn deref (& self) -> & G { self . 0 } }
};
}
