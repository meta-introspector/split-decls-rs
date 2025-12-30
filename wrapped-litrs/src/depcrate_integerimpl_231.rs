// Generated macro for impl_231 (impl)
macro_rules! Depcrate_integerimpl_231 {
() => {
// Module: crate::integer
// Provides: {"impl_231"}
// Dependencies: {}
impl FromStr for IntegerType { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: from_suffix (s) . ok_or (()) } }
};
}
