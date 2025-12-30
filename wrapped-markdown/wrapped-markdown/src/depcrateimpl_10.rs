// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Opts { pub fn build (& self) -> Box < dyn WorldGenerator > { let mut r = Markdown :: default () ; r . opts = self . clone () ; Box :: new (r) } }
};
}
