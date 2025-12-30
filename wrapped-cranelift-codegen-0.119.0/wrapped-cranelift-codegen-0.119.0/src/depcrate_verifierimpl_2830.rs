// Generated macro for impl_2830 (impl)
macro_rules! Depcrate_verifierimpl_2830 {
() => {
// Module: crate::verifier
// Provides: {"impl_2830"}
// Dependencies: {}
# [doc = " Convenience converter for making error-reporting less verbose."] # [doc = ""] # [doc = " Converts a tuple of `(location, context, message)` to a `VerifierError`."] # [doc = " ```"] # [doc = " use cranelift_codegen::verifier::VerifierErrors;"] # [doc = " use cranelift_codegen::ir::Inst;"] # [doc = " let mut errors = VerifierErrors::new();"] # [doc = " errors.report((Inst::from_u32(42), \"v3 = iadd v1, v2\", \"iadd cannot be used with values of this type\"));"] # [doc = " // note the double parenthenses to use this syntax"] # [doc = " ```"] impl < L , C , M > From < (L , C , M) > for VerifierError where L : Into < AnyEntity > , C : Into < String > , M : Into < String > , { fn from (items : (L , C , M)) -> Self { let (location , context , message) = items ; Self { location : location . into () , context : Some (context . into ()) , message : message . into () , } } }
};
}
