// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl BuildMode { # [doc = " Returns a string with the cargo command matching this build mode."] pub fn as_str (& self) -> & str { use BuildMode :: * ; match self { Build => "build" , Check => "check" , } } }
};
}
