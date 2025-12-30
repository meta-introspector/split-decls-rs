// Generated macro for impl_846 (impl)
macro_rules! Depcrate_strategy_fuseimpl_846 {
() => {
// Module: crate::strategy::fuse
// Provides: {"impl_846"}
// Dependencies: {}
impl < T : ValueTree > ValueTree for Fuse < T > { type Value = T :: Value ; fn current (& self) -> T :: Value { self . inner . current () } fn simplify (& mut self) -> bool { if self . may_simplify { if self . inner . simplify () { self . may_complicate = true ; true } else { self . may_simplify = false ; false } } else { false } } fn complicate (& mut self) -> bool { if self . may_complicate { if self . inner . complicate () { self . may_simplify = true ; true } else { self . may_complicate = false ; false } } else { false } } }
};
}
