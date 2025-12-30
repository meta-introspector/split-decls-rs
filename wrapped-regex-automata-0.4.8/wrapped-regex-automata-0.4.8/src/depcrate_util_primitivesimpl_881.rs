// Generated macro for impl_881 (impl)
macro_rules! Depcrate_util_primitivesimpl_881 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_881"}
// Dependencies: {}
impl TryFrom < u64 > for SmallIndex { type Error = SmallIndexError ; fn try_from (index : u64) -> Result < SmallIndex , SmallIndexError > { if index > SmallIndex :: MAX . as_u64 () { return Err (SmallIndexError { attempted : index }) ; } Ok (SmallIndex :: new_unchecked (index . as_usize ())) } }
};
}
