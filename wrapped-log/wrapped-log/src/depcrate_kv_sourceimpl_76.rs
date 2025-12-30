// Generated macro for impl_76 (impl)
macro_rules! Depcrate_kv_sourceimpl_76 {
() => {
// Module: crate::kv::source
// Provides: {"impl_76"}
// Dependencies: {}
impl < S > Source for Option < S > where S : Source , { fn visit < 'kvs > (& 'kvs self , visitor : & mut dyn VisitSource < 'kvs >) -> Result < () , Error > { if let Some (source) = self { source . visit (visitor) ? ; } Ok (()) } fn get (& self , key : Key) -> Option < Value < '_ > > { self . as_ref () . and_then (| s | s . get (key)) } fn count (& self) -> usize { self . as_ref () . map_or (0 , Source :: count) } }
};
}
