// Generated macro for impl_71 (impl)
macro_rules! Depcrate_search_outcomeimpl_71 {
() => {
// Module: crate::search::outcome
// Provides: {"impl_71"}
// Dependencies: {}
impl Match { fn to_outer < 'a > (& self , out : & 'a Outcome) -> crate :: search :: Match < 'a > { crate :: search :: Match { pattern : out . patterns . resolve (self . pattern) . expect ("pattern still present") , assignment : out . assignments . resolve (self . assignment) . expect ("assignment present") . as_ref () , kind : self . kind , location : self . location . to_outer (out) , } } }
};
}
