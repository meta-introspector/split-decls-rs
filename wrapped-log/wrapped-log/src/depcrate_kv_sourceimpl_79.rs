// Generated macro for impl_79 (impl)
macro_rules! Depcrate_kv_sourceimpl_79 {
() => {
// Module: crate::kv::source
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a , 'b : 'a , 'kvs > VisitSource < 'kvs > for fmt :: DebugMap < 'a , 'b > { fn visit_pair (& mut self , key : Key < 'kvs > , value : Value < 'kvs >) -> Result < () , Error > { self . entry (& key , & value) ; Ok (()) } }
};
}
