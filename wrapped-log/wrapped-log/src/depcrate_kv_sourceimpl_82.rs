// Generated macro for impl_82 (impl)
macro_rules! Depcrate_kv_sourceimpl_82 {
() => {
// Module: crate::kv::source
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a , 'b : 'a , 'kvs > VisitSource < 'kvs > for fmt :: DebugTuple < 'a , 'b > { fn visit_pair (& mut self , key : Key < 'kvs > , value : Value < 'kvs >) -> Result < () , Error > { self . field (& key) ; self . field (& value) ; Ok (()) } }
};
}
