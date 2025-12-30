// Generated macro for impl_417 (impl)
macro_rules! Depcrate_sourcesimpl_417 {
() => {
// Module: crate::sources
// Provides: {"impl_417"}
// Dependencies: {}
impl < F : Fn () -> Option < String > > Source for LazySource < F > { type Item = String ; type Error = IOError ; fn load (& self) -> Result < Self :: Item , Self :: Error > { (self . loader) () . ok_or (IOError :: other ("Source load error")) } }
};
}
