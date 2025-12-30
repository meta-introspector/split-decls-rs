// Generated macro for impl_89 (impl)
macro_rules! Depcrate_namesimpl_89 {
() => {
// Module: crate::names
// Provides: {"impl_89"}
// Dependencies: {}
impl PropertyEnumToValueNameLookup for PropertyScriptToIcuScriptMap < '_ > { fn get (& self , prop : u32) -> Option < & str > { self . map . get_ule_ref (usize :: try_from (prop) . ok () ?) . and_then (| no | no . as_ref ()) . map (| s | s . as_str ()) } }
};
}
