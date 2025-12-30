// Generated macro for impl_69 (impl)
macro_rules! Depcrate_kv_sourceimpl_69 {
() => {
// Module: crate::kv::source
// Provides: {"impl_69"}
// Dependencies: {}
impl < 'a , T > Source for & 'a T where T : Source + ? Sized , { fn visit < 'kvs > (& 'kvs self , visitor : & mut dyn VisitSource < 'kvs >) -> Result < () , Error > { Source :: visit (& * * self , visitor) } fn get (& self , key : Key) -> Option < Value < '_ > > { Source :: get (& * * self , key) } fn count (& self) -> usize { Source :: count (& * * self) } }
};
}
