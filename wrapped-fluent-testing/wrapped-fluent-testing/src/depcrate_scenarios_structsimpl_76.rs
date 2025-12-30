// Generated macro for impl_76 (impl)
macro_rules! Depcrate_scenarios_structsimpl_76 {
() => {
// Module: crate::scenarios::structs
// Provides: {"impl_76"}
// Dependencies: {}
impl Scenario { pub fn new < S : ToString , R : Into < ResourceId > , Q : Into < Queries > > (name : S , file_sources : Vec < FileSource > , locales : Vec < S > , res_ids : Vec < R > , queries : Q ,) -> Self { Self { name : name . to_string () , file_sources , locales : locales . into_iter () . map (| l | l . to_string () . parse () . unwrap ()) . collect () , res_ids : res_ids . into_iter () . map (| id | id . into ()) . collect () , queries : queries . into () , } } }
};
}
