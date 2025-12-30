// Generated macro for impl_77 (impl)
macro_rules! Depcrate_kv_sourceimpl_77 {
() => {
// Module: crate::kv::source
// Provides: {"impl_77"}
// Dependencies: {}
impl < 'a , 'b : 'a , 'kvs > VisitSource < 'kvs > for fmt :: DebugList < 'a , 'b > { fn visit_pair (& mut self , key : Key < 'kvs > , value : Value < 'kvs >) -> Result < () , Error > { self . entry (& (key , value)) ; Ok (()) } }
};
}
