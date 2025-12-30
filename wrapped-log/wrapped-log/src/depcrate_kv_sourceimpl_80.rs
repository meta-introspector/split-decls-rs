// Generated macro for impl_80 (impl)
macro_rules! Depcrate_kv_sourceimpl_80 {
() => {
// Module: crate::kv::source
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a , 'b : 'a , 'kvs > VisitSource < 'kvs > for fmt :: DebugList < 'a , 'b > { fn visit_pair (& mut self , key : Key < 'kvs > , value : Value < 'kvs >) -> Result < () , Error > { self . entry (& (key , value)) ; Ok (()) } }
};
}
