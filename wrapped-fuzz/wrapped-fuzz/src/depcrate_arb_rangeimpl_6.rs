// Generated macro for impl_6 (impl)
macro_rules! Depcrate_arb_rangeimpl_6 {
() => {
// Module: crate::arb_range
// Provides: {"impl_6"}
// Dependencies: {}
impl < T : Arbitrary > Arbitrary for ArbRange < T > { fn arbitrary (u : & mut Unstructured) -> Result < Self > { let variant = u8 :: arbitrary (u) ? % 5 ; Ok (match variant { 0 => ArbRange :: Range (T :: arbitrary (u) ? .. T :: arbitrary (u) ?) , 1 => ArbRange :: RangeFrom (T :: arbitrary (u) ? ..) , 2 => ArbRange :: RangeInclusive (T :: arbitrary (u) ? ..= T :: arbitrary (u) ?) , 3 => ArbRange :: RangeTo (.. T :: arbitrary (u) ?) , 4 => ArbRange :: RangeToInclusive (..= T :: arbitrary (u) ?) , _ => unreachable ! () , }) } }
};
}
