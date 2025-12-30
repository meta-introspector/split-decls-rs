// Generated macro for impl_340 (impl)
macro_rules! Depcrate_quartzcoreimpl_340 {
() => {
// Module: crate::quartzcore
// Provides: {"impl_340"}
// Dependencies: {}
impl Filter { fn into_CFString (self) -> CFString { let string = match self { Filter :: Nearest => "nearest" , Filter :: Linear => "linear" , Filter :: Trilinear => "trilinear" , Filter :: Other (other) => return other , } ; CFString :: from (string) } fn from_CFString (string : CFString) -> Filter { match string . to_string () { ref s if s == "nearest" => Filter :: Nearest , ref s if s == "linear" => Filter :: Linear , ref s if s == "trilinear" => Filter :: Trilinear , _ => Filter :: Other (string) , } } }
};
}
