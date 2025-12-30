// Generated macro for impl_129 (impl)
macro_rules! Depcrate_parse_argumentsimpl_129 {
() => {
// Module: crate::parse::arguments
// Provides: {"impl_129"}
// Dependencies: {}
impl Args { fn get (& self , pat : & Pat) -> Option < & ArgumentInfo > { self . args . get (pat) . or_else (| | self . args . get (& pat_invert_mutability (pat))) } fn entry (& mut self , pat : Pat) -> std :: collections :: hash_map :: Entry < '_ , Pat , ArgumentInfo > { self . args . entry (pat) } }
};
}
