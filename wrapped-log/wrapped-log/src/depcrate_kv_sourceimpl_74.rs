// Generated macro for impl_74 (impl)
macro_rules! Depcrate_kv_sourceimpl_74 {
() => {
// Module: crate::kv::source
// Provides: {"impl_74"}
// Dependencies: {}
impl < S > Source for [S] where S : Source , { fn visit < 'kvs > (& 'kvs self , visitor : & mut dyn VisitSource < 'kvs >) -> Result < () , Error > { for source in self { source . visit (visitor) ? ; } Ok (()) } fn get (& self , key : Key) -> Option < Value < '_ > > { for source in self { if let Some (found) = source . get (key . clone ()) { return Some (found) ; } } None } fn count (& self) -> usize { self . iter () . map (Source :: count) . sum () } }
};
}
