// Generated macro for impl_71 (impl)
macro_rules! Depcrate_kv_sourceimpl_71 {
() => {
// Module: crate::kv::source
// Provides: {"impl_71"}
// Dependencies: {}
impl < S > Source for [S] where S : Source , { fn visit < 'kvs > (& 'kvs self , visitor : & mut dyn VisitSource < 'kvs >) -> Result < () , Error > { for source in self { source . visit (visitor) ? ; } Ok (()) } fn get (& self , key : Key) -> Option < Value < '_ > > { for source in self { if let Some (found) = source . get (key . clone ()) { return Some (found) ; } } None } fn count (& self) -> usize { self . iter () . map (Source :: count) . sum () } }
};
}
