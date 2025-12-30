// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < T : Clone + PartialOrd + PartialEq + Debug > PartialOrd for AnyRange < T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { match (self , other) { (AnyRange :: Range (a) , AnyRange :: Range (b)) if a . start == b . start => { a . end . partial_cmp (& b . end) } (AnyRange :: Range (a) , AnyRange :: Range (b)) => a . start . partial_cmp (& b . start) , (AnyRange :: RangeFrom (a) , AnyRange :: RangeFrom (b)) => a . start . partial_cmp (& b . start) , (AnyRange :: RangeFull (_) , AnyRange :: RangeFull (_)) => Some (Ordering :: Equal) , (AnyRange :: RangeInclusive (a) , AnyRange :: RangeInclusive (b)) if a . start () == b . start () => { a . end () . partial_cmp (b . end ()) } (AnyRange :: RangeInclusive (a) , AnyRange :: RangeInclusive (b)) => { a . start () . partial_cmp (b . start ()) } (AnyRange :: RangeTo (a) , AnyRange :: RangeTo (b)) => a . end . partial_cmp (& b . end) , (AnyRange :: RangeToInclusive (a) , AnyRange :: RangeToInclusive (b)) => { a . end . partial_cmp (& b . end) } (a , b) => a . order () . partial_cmp (& b . order ()) , } } }
};
}
