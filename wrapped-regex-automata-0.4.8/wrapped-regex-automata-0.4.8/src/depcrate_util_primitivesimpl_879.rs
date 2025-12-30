// Generated macro for impl_879 (impl)
macro_rules! Depcrate_util_primitivesimpl_879 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_879"}
// Dependencies: {}
impl TryFrom < u16 > for SmallIndex { type Error = SmallIndexError ; fn try_from (index : u16) -> Result < SmallIndex , SmallIndexError > { if u32 :: from (index) > SmallIndex :: MAX . as_u32 () { return Err (SmallIndexError { attempted : u64 :: from (index) }) ; } Ok (SmallIndex :: new_unchecked (index . as_usize ())) } }
};
}
