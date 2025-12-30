// Generated macro for impl_73 (impl)
macro_rules! Depcrate_scenarios_structsimpl_73 {
() => {
// Module: crate::scenarios::structs
// Provides: {"impl_73"}
// Dependencies: {}
impl From < Vec < (& str , & str) > > for Queries { fn from (input : Vec < (& str , & str) >) -> Self { Self (input . into_iter () . map (| q | q . into ()) . collect ()) } }
};
}
