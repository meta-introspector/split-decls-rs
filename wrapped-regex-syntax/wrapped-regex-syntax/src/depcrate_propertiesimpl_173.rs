// Generated macro for impl_173 (impl)
macro_rules! Depcrate_propertiesimpl_173 {
() => {
// Module: crate::properties
// Provides: {"impl_173"}
// Dependencies: {}
impl Arbitrary for CharClass { fn arbitrary < G : Gen > (g : & mut G) -> CharClass { let mut ranges : Vec < ClassRange > = Arbitrary :: arbitrary (g) ; if ranges . is_empty () { ranges . push (Arbitrary :: arbitrary (g)) ; } let cls = CharClass { ranges : ranges } . canonicalize () ; if g . gen () { cls . case_fold () } else { cls } } fn shrink (& self) -> Box < Iterator < Item = CharClass > > { Box :: new (self . ranges . clone () . shrink () . filter (| ranges | ranges . len () > 0) . map (| ranges | CharClass { ranges : ranges } . canonicalize ())) } }
};
}
