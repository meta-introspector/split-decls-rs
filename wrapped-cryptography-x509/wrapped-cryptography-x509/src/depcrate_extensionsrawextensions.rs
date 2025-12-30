// Generated macro for RawExtensions (type)
macro_rules! Depcrate_extensionsRawExtensions {
() => {
// Module: crate::extensions
// Provides: {"RawExtensions"}
// Dependencies: {}
pub type RawExtensions < 'a > = common :: Asn1ReadableOrWritable < asn1 :: SequenceOf < 'a , Extension < 'a > > , asn1 :: SequenceOfWriter < 'a , Extension < 'a > , Vec < Extension < 'a > > > , > ;
};
}
