// Generated macro for impl_70 (impl)
macro_rules! Depcrate_kv_sourceimpl_70 {
() => {
// Module: crate::kv::source
// Provides: {"impl_70"}
// Dependencies: {}
impl < K , V > Source for (K , V) where K : ToKey , V : ToValue , { fn visit < 'kvs > (& 'kvs self , visitor : & mut dyn VisitSource < 'kvs >) -> Result < () , Error > { visitor . visit_pair (self . 0 . to_key () , self . 1 . to_value ()) } fn get (& self , key : Key) -> Option < Value < '_ > > { if self . 0 . to_key () == key { Some (self . 1 . to_value ()) } else { None } } fn count (& self) -> usize { 1 } }
};
}
