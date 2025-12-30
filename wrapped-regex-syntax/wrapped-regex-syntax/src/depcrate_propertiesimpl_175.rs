// Generated macro for impl_175 (impl)
macro_rules! Depcrate_propertiesimpl_175 {
() => {
// Module: crate::properties
// Provides: {"impl_175"}
// Dependencies: {}
impl Arbitrary for ByteClass { fn arbitrary < G : Gen > (g : & mut G) -> ByteClass { let mut ranges : Vec < ByteRange > = Arbitrary :: arbitrary (g) ; if ranges . is_empty () { ranges . push (Arbitrary :: arbitrary (g)) ; } let cls = ByteClass { ranges : ranges } . canonicalize () ; if g . gen () { cls . case_fold () } else { cls } } fn shrink (& self) -> Box < Iterator < Item = ByteClass > > { Box :: new (self . ranges . clone () . shrink () . filter (| ranges | ranges . len () > 0) . map (| ranges | ByteClass { ranges : ranges } . canonicalize ())) } }
};
}
