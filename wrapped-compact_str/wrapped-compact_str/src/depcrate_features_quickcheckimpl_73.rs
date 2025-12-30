// Generated macro for impl_73 (impl)
macro_rules! Depcrate_features_quickcheckimpl_73 {
() => {
// Module: crate::features::quickcheck
// Provides: {"impl_73"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "quickcheck")))] impl Arbitrary for CompactString { fn arbitrary (g : & mut Gen) -> CompactString { let max = g . size () ; let x = usize :: arbitrary (g) ; let ratio = (x as f64) / (usize :: MAX as f64) ; let size = (ratio * max as f64) as usize ; (0 .. size) . map (| _ | char :: arbitrary (g)) . collect () } fn shrink (& self) -> Box < dyn Iterator < Item = CompactString > > { let chars : Vec < char > = self . chars () . collect () ; Box :: new (chars . shrink () . map (| x | x . into_iter () . collect :: < CompactString > ()) ,) } }
};
}
