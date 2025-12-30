// Generated macro for impl_1387 (impl)
macro_rules! Depcrate_sampleimpl_1387 {
() => {
// Module: crate::sample
// Provides: {"impl_1387"}
// Dependencies: {}
impl < T : fmt :: Debug + Clone + 'static > ValueTree for SubsequenceValueTree < T > { type Value = Vec < T > ; fn current (& self) -> Self :: Value { let inner = self . inner . current () ; let ret = inner . iter () . map (| ix | self . values [ix] . clone ()) . collect () ; ret } fn simplify (& mut self) -> bool { self . inner . simplify () } fn complicate (& mut self) -> bool { self . inner . complicate () } }
};
}
