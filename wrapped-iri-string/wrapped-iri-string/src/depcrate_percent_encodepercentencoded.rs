// Generated macro for PercentEncoded (struct)
macro_rules! Depcrate_percent_encodePercentEncoded {
() => {
// Module: crate::percent_encode
// Provides: {"PercentEncoded"}
// Dependencies: {}
# [doc = " A proxy to percent-encode a string."] # [doc = ""] # [doc = " Type aliases [`PercentEncodedForIri`] and [`PercentEncodedForUri`] are provided."] # [doc = " You can use them to make the expression simpler, for example write"] # [doc = " `PercentEncodedForUri::from_path(foo)` instead of"] # [doc = " `PercentEncoded::<_, UriSpec>::from_path(foo)`."] # [derive (Debug , Clone , Copy)] pub struct PercentEncoded < T , S > { # [doc = " Source string context."] context : Context , # [doc = " Raw string before being encoded."] raw : T , # [doc = " Spec."] _spec : PhantomData < fn () -> S > , }
};
}
