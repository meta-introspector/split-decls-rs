// Generated macro for impl_128 (impl)
macro_rules! Depcrateimpl_128 {
() => {
// Module: crate
// Provides: {"impl_128"}
// Dependencies: {}
impl < T > FromStr for Complex < T > where T : FromStr + Num + Clone , { type Err = ParseComplexError < T :: Err > ; # [doc = " Parses `a +/- bi`; `ai +/- b`; `a`; or `bi` where `a` and `b` are of type `T`"] fn from_str (s : & str) -> Result < Self , Self :: Err > { from_str_generic (s , T :: from_str) } }
};
}
