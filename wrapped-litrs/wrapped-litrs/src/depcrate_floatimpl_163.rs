// Generated macro for impl_163 (impl)
macro_rules! Depcrate_floatimpl_163 {
() => {
// Module: crate::float
// Provides: {"impl_163"}
// Dependencies: {}
impl FromStr for FloatType { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: from_suffix (s) . ok_or (()) } }
};
}
