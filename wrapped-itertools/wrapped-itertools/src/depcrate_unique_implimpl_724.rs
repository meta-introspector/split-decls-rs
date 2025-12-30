// Generated macro for impl_724 (impl)
macro_rules! Depcrate_unique_implimpl_724 {
() => {
// Module: crate::unique_impl
// Provides: {"impl_724"}
// Dependencies: {}
impl < I > fmt :: Debug for Unique < I > where I : Iterator + fmt :: Debug , I :: Item : Hash + Eq + fmt :: Debug + Clone , { debug_fmt_fields ! (Unique , iter) ; }
};
}
