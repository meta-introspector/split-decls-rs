// Generated macro for impl_714 (impl)
macro_rules! Depcrate_unique_implimpl_714 {
() => {
// Module: crate::unique_impl
// Provides: {"impl_714"}
// Dependencies: {}
impl < I , V , F > fmt :: Debug for UniqueBy < I , V , F > where I : Iterator + fmt :: Debug , V : fmt :: Debug + Hash + Eq , { debug_fmt_fields ! (UniqueBy , iter , used) ; }
};
}
