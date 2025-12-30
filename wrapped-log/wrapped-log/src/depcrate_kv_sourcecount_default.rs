// Generated macro for count_default (function)
macro_rules! Depcrate_kv_sourcecount_default {
() => {
// Module: crate::kv::source
// Provides: {"count_default"}
// Dependencies: {}
# [doc = " The default implementation of `Source::count`."] fn count_default (source : impl Source) -> usize { struct Count (usize) ; impl < 'kvs > VisitSource < 'kvs > for Count { fn visit_pair (& mut self , _ : Key < 'kvs > , _ : Value < 'kvs >) -> Result < () , Error > { self . 0 += 1 ; Ok (()) } } let mut count = Count (0) ; let _ = source . visit (& mut count) ; count . 0 }
};
}
