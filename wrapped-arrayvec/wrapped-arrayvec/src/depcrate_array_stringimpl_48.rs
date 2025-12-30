// Generated macro for impl_48 (impl)
macro_rules! Depcrate_array_stringimpl_48 {
() => {
// Module: crate::array_string
// Provides: {"impl_48"}
// Dependencies: {}
impl < const CAP : usize > FromStr for ArrayString < CAP > { type Err = CapacityError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: from (s) . map_err (CapacityError :: simplify) } }
};
}
