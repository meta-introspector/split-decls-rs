// Generated macro for impl_35 (impl)
macro_rules! Depcrate_opimpl_35 {
() => {
// Module: crate::op
// Provides: {"impl_35"}
// Dependencies: {}
# [cfg (not (feature = "regex"))] impl FilterOp { pub fn new (spec : & str) -> Result < Self , String > { Ok (Self { inner : spec . to_string () , }) } pub fn is_match (& self , s : & str) -> bool { s . contains (& self . inner) } }
};
}
