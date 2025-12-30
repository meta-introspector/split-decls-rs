// Generated macro for impl_50 (impl)
macro_rules! Depcrate_scenarios_structsimpl_50 {
() => {
// Module: crate::scenarios::structs
// Provides: {"impl_50"}
// Dependencies: {}
impl FileSource { pub fn new < S : ToString > (name : S , path_scheme : S , locales : Vec < S >) -> Self { Self { name : name . to_string () , path_scheme : path_scheme . to_string () , locales : locales . iter () . map (| l | l . to_string () . parse () . unwrap ()) . collect () , } } }
};
}
