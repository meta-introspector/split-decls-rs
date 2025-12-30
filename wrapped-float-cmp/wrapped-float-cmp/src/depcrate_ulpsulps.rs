// Generated macro for Ulps (trait)
macro_rules! Depcrate_ulpsUlps {
() => {
// Module: crate::ulps
// Provides: {"Ulps"}
// Dependencies: {}
# [cfg (not (feature = "num-traits"))] pub trait Ulps { type U : Copy ; # [doc = " The number of representable values or ULPs (Units of Least Precision) that"] # [doc = " separate `self` and `other`.  The result `U` is an integral value, and will"] # [doc = " be zero if `self` and `other` are exactly equal."] fn ulps (& self , other : & Self) -> < Self as Ulps > :: U ; # [doc = " The next representable number above this one"] fn next (& self) -> Self ; # [doc = " The previous representable number below this one"] fn prev (& self) -> Self ; }
};
}
