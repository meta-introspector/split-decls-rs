// Generated macro for impl_70 (impl)
macro_rules! Depcrate_type_impl_70 {
() => {
// Module: crate::type_
// Provides: {"impl_70"}
// Dependencies: {}
impl FromStr for MediaType { type Err = InvalidMime ; fn from_str (s : & str) -> Result < MediaType , Self :: Err > { MediaType :: parse (s) } }
};
}
