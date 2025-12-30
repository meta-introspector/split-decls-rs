// Generated macro for Attributes (type)
macro_rules! Depcrate_csrAttributes {
() => {
// Module: crate::csr
// Provides: {"Attributes"}
// Dependencies: {}
pub type Attributes < 'a > = common :: Asn1ReadableOrWritable < asn1 :: SetOf < 'a , Attribute < 'a > > , asn1 :: SetOfWriter < 'a , Attribute < 'a > , Vec < Attribute < 'a > > > , > ;
};
}
