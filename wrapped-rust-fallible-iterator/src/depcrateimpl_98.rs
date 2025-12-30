// Generated macro for impl_98 (impl)
macro_rules! Depcrateimpl_98 {
() => {
// Module: crate
// Provides: {"impl_98"}
// Dependencies: {}
impl < I , E , F > FallibleIterator for FromFn < F > where F : FnMut () -> Result < Option < I > , E > , { type Item = I ; type Error = E ; fn next (& mut self) -> Result < Option < I > , E > { (self . fun) () } }
};
}
