// Generated macro for impl_275 (impl)
macro_rules! Depcrate_parse_sectionimpl_275 {
() => {
// Module: crate::parse::section
// Provides: {"impl_275"}
// Dependencies: {}
impl Section < '_ > { # [doc = " Turn this instance into a fully owned one with `'static` lifetime."] # [must_use] pub fn to_owned (& self) -> Section < 'static > { Section { header : self . header . to_owned () , events : self . events . iter () . map (Event :: to_owned) . collect () , } } }
};
}
