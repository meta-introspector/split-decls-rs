// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl AsRef < str > for Source { fn as_ref (& self) -> & str { match * self { Source :: Atom (_ , s) => s , Source :: Dynamic (ref s) => s , } } }
};
}
