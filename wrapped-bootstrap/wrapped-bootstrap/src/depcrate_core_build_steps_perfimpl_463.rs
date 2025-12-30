// Generated macro for impl_463 (impl)
macro_rules! Depcrate_core_build_steps_perfimpl_463 {
() => {
// Module: crate::core::build_steps::perf
// Provides: {"impl_463"}
// Dependencies: {}
impl Display for Profile { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { let name = match self { Profile :: Check => "Check" , Profile :: Debug => "Debug" , Profile :: Doc => "Doc" , Profile :: Opt => "Opt" , Profile :: Clippy => "Clippy" , } ; f . write_str (name) } }
};
}
