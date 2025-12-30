// Generated macro for RInto (trait)
macro_rules! Depcrate_util_rangeintRInto {
() => {
// Module: crate::util::rangeint
// Provides: {"RInto"}
// Dependencies: {}
# [doc = " A trait for losslessly converting to ranged integers."] # [doc = ""] # [doc = " This goes along with `RFrom` and exists to make things like `t.rinto()`"] # [doc = " work without the need to do `T::rfrom(..)`. Like the standard library"] # [doc = " `Into` trait, a blanket impl is provided based on impls of `RFrom`. Callers"] # [doc = " are not expected to implement this trait directly."] pub (crate) trait RInto < T > : Sized { fn rinto (self) -> T ; }
};
}
