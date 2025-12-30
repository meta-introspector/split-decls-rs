// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl Opts { pub fn build (self) -> Box < dyn WorldGenerator > { let mut r = RustWasm :: new () ; r . skip = self . skip . iter () . cloned () . collect () ; r . opts = self ; Box :: new (r) } }
};
}
