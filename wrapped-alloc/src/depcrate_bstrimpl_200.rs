// Generated macro for impl_200 (impl)
macro_rules! Depcrate_bstrimpl_200 {
() => {
// Module: crate::bstr
// Provides: {"impl_200"}
// Dependencies: {}
# [unstable (feature = "bstr" , issue = "134915")] impl FromStr for ByteString { type Err = core :: convert :: Infallible ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (ByteString (s . as_bytes () . to_vec ())) } }
};
}
