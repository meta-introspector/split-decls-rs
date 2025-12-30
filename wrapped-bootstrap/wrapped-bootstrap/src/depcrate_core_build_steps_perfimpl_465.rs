// Generated macro for impl_465 (impl)
macro_rules! Depcrate_core_build_steps_perfimpl_465 {
() => {
// Module: crate::core::build_steps::perf
// Provides: {"impl_465"}
// Dependencies: {}
impl Display for Scenario { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { let name = match self { Scenario :: Full => "Full" , Scenario :: IncrFull => "IncrFull" , Scenario :: IncrUnchanged => "IncrUnchanged" , Scenario :: IncrPatched => "IncrPatched" , } ; f . write_str (name) } }
};
}
