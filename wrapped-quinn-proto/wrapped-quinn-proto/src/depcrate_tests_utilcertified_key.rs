// Generated macro for CERTIFIED_KEY (static)
macro_rules! Depcrate_tests_utilCERTIFIED_KEY {
() => {
// Module: crate::tests::util
// Provides: {"CERTIFIED_KEY"}
// Dependencies: {}
pub (crate) static CERTIFIED_KEY : LazyLock < rcgen :: CertifiedKey < rcgen :: KeyPair > > = LazyLock :: new (| | rcgen :: generate_simple_self_signed (vec ! ["localhost" . into ()]) . unwrap ()) ;
};
}
