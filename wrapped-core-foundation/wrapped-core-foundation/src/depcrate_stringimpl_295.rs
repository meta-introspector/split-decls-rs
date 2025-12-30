// Generated macro for impl_295 (impl)
macro_rules! Depcrate_stringimpl_295 {
() => {
// Module: crate::string
// Provides: {"impl_295"}
// Dependencies: {}
impl FromStr for CFString { type Err = () ; # [doc = " See also [`CFString::new()`] for a variant of this which does not return a `Result`."] # [inline] fn from_str (string : & str) -> Result < CFString , () > { Ok (CFString :: new (string)) } }
};
}
