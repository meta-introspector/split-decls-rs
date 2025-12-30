// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < T : Clone + PartialOrd + PartialEq + Debug > Debug for AnyRange < T > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { match self { AnyRange :: Range (r) => write ! (f , "AnyRange({r:?})") , AnyRange :: RangeFrom (r) => write ! (f , "AnyRange({r:?})") , AnyRange :: RangeFull (r) => write ! (f , "AnyRange({r:?})") , AnyRange :: RangeInclusive (r) => write ! (f , "AnyRange({r:?})") , AnyRange :: RangeTo (r) => write ! (f , "AnyRange({r:?})") , AnyRange :: RangeToInclusive (r) => write ! (f , "AnyRange({r:?})") , } } }
};
}
