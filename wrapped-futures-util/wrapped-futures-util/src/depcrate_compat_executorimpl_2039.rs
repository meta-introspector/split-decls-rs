// Generated macro for impl_2039 (impl)
macro_rules! Depcrate_compat_executorimpl_2039 {
() => {
// Module: crate::compat::executor
// Provides: {"impl_2039"}
// Dependencies: {}
impl < Ex > Executor01CompatExt for Ex where Ex : Executor01 < Executor01Future > + Clone + Send + 'static , { fn compat (self) -> Executor01As03 < Self > { Executor01As03 { executor01 : self } } }
};
}
