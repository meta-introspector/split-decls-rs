// Generated macro for impl_81 (impl)
macro_rules! Depcrate_kv_sourceimpl_81 {
() => {
// Module: crate::kv::source
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a , 'b : 'a , 'kvs > VisitSource < 'kvs > for fmt :: DebugSet < 'a , 'b > { fn visit_pair (& mut self , key : Key < 'kvs > , value : Value < 'kvs >) -> Result < () , Error > { self . entry (& (key , value)) ; Ok (()) } }
};
}
