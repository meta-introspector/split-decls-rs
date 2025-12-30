// Generated macro for impl_78 (impl)
macro_rules! Depcrate_kv_sourceimpl_78 {
() => {
// Module: crate::kv::source
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a , 'kvs , T > VisitSource < 'kvs > for & 'a mut T where T : VisitSource < 'kvs > + ? Sized , { fn visit_pair (& mut self , key : Key < 'kvs > , value : Value < 'kvs >) -> Result < () , Error > { (* * self) . visit_pair (key , value) } }
};
}
