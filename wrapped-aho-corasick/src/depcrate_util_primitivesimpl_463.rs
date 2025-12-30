// Generated macro for impl_463 (impl)
macro_rules! Depcrate_util_primitivesimpl_463 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_463"}
// Dependencies: {}
impl TryFrom < u32 > for SmallIndex { type Error = SmallIndexError ; fn try_from (index : u32) -> Result < SmallIndex , SmallIndexError > { if index > SmallIndex :: MAX . as_u32 () { return Err (SmallIndexError { attempted : u64 :: from (index) }) ; } Ok (SmallIndex :: new_unchecked (index . as_usize ())) } }
};
}
