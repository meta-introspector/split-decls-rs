// Generated macro for impl_433 (impl)
macro_rules! Depcrate_validation_visitorimpl_433 {
() => {
// Module: crate::validation::visitor
// Provides: {"impl_433"}
// Dependencies: {}
impl From < RuleError > for ServerError { fn from (e : RuleError) -> Self { Self { message : e . message , source : None , locations : e . locations , path : Vec :: new () , extensions : None , } } }
};
}
