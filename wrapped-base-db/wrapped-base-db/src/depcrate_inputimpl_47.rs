// Generated macro for impl_47 (impl)
macro_rules! Depcrate_inputimpl_47 {
() => {
// Module: crate::input
// Provides: {"impl_47"}
// Dependencies: {}
impl CrateOrigin { pub fn is_local (& self) -> bool { matches ! (self , CrateOrigin :: Local { .. }) } pub fn is_lib (& self) -> bool { matches ! (self , CrateOrigin :: Library { .. }) } pub fn is_lang (& self) -> bool { matches ! (self , CrateOrigin :: Lang { .. }) } }
};
}
