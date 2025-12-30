// Generated macro for MaxSize (type)
macro_rules! Depcrate_derMaxSize {
() => {
// Module: crate::der
// Provides: {"MaxSize"}
// Dependencies: {}
# [doc = " Maximum size of an ASN.1 DER encoded signature for the given elliptic curve."] pub type MaxSize < C > = < < FieldBytesSize < C > as Add > :: Output as Add < MaxOverhead > > :: Output ;
};
}
