// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl CheckRaw for CStr { type RawUnit = NonZero < char > ; # [inline] fn char2raw_unit (c : char) -> Result < Self :: RawUnit , EscapeError > { NonZero :: new (c) . ok_or (EscapeError :: NulInCStr) } }
};
}
