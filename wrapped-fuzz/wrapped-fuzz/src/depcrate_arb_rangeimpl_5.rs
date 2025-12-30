// Generated macro for impl_5 (impl)
macro_rules! Depcrate_arb_rangeimpl_5 {
() => {
// Module: crate::arb_range
// Provides: {"impl_5"}
// Dependencies: {}
impl < T > RangeBounds < T > for ArbRange < T > { fn start_bound (& self) -> Bound < & T > { match self { ArbRange :: Range (range) => range . start_bound () , ArbRange :: RangeFrom (range) => range . start_bound () , ArbRange :: RangeInclusive (range) => range . start_bound () , ArbRange :: RangeTo (range) => range . start_bound () , ArbRange :: RangeToInclusive (range) => range . start_bound () , } } fn end_bound (& self) -> Bound < & T > { match self { ArbRange :: Range (range) => range . end_bound () , ArbRange :: RangeFrom (range) => range . end_bound () , ArbRange :: RangeInclusive (range) => range . end_bound () , ArbRange :: RangeTo (range) => range . end_bound () , ArbRange :: RangeToInclusive (range) => range . end_bound () , } } fn contains < U : ? Sized > (& self , item : & U) -> bool where T : PartialOrd < U > , U : PartialOrd < T > , { match self { ArbRange :: Range (range) => range . contains (item) , ArbRange :: RangeFrom (range) => range . contains (item) , ArbRange :: RangeInclusive (range) => range . contains (item) , ArbRange :: RangeTo (range) => range . contains (item) , ArbRange :: RangeToInclusive (range) => range . contains (item) , } } }
};
}
