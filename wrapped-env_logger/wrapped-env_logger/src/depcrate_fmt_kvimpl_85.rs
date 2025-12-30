// Generated macro for impl_85 (impl)
macro_rules! Depcrate_fmt_kvimpl_85 {
() => {
// Module: crate::fmt::kv
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'kvs > VisitSource < 'kvs > for DefaultVisitSource < '_ > { fn visit_pair (& mut self , key : Key < '_ > , value : Value < 'kvs >) -> Result < () , Error > { write ! (self . 0 , " {}={}" , self . style_key (key) , value) ? ; Ok (()) } }
};
}
