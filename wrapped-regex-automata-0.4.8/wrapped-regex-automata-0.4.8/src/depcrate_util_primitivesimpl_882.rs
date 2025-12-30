// Generated macro for impl_882 (impl)
macro_rules! Depcrate_util_primitivesimpl_882 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_882"}
// Dependencies: {}
impl TryFrom < usize > for SmallIndex { type Error = SmallIndexError ; fn try_from (index : usize) -> Result < SmallIndex , SmallIndexError > { if index > SmallIndex :: MAX . as_usize () { return Err (SmallIndexError { attempted : index . as_u64 () }) ; } Ok (SmallIndex :: new_unchecked (index)) } }
};
}
