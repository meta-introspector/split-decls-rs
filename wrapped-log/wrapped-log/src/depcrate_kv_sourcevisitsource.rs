// Generated macro for VisitSource (trait)
macro_rules! Depcrate_kv_sourceVisitSource {
() => {
// Module: crate::kv::source
// Provides: {"VisitSource"}
// Dependencies: {}
# [doc = " A visitor for the key-value pairs in a [`Source`](trait.Source.html)."] pub trait VisitSource < 'kvs > { # [doc = " Visit a key-value pair."] fn visit_pair (& mut self , key : Key < 'kvs > , value : Value < 'kvs >) -> Result < () , Error > ; }
};
}
