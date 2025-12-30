// Generated macro for impl_414 (impl)
macro_rules! Depcrate_sourcesimpl_414 {
() => {
// Module: crate::sources
// Provides: {"impl_414"}
// Dependencies: {}
impl Source for FileSource { type Item = String ; type Error = IOError ; fn load (& self) -> Result < Self :: Item , Self :: Error > { let mut reader = BufReader :: new (File :: open (& self . path) ?) ; let mut buf = String :: new () ; reader . read_to_string (& mut buf) ? ; Ok (buf) } }
};
}
