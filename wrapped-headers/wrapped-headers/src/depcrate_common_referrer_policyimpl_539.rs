// Generated macro for impl_539 (impl)
macro_rules! Depcrate_common_referrer_policyimpl_539 {
() => {
// Module: crate::common::referrer_policy
// Provides: {"impl_539"}
// Dependencies: {}
impl TryFromValues for Policy { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { let mut known = None ; for s in csv (values) { known = Some (match s { "no-referrer" | "never" => Policy :: NoReferrer , "no-referrer-when-downgrade" | "default" => Policy :: NoReferrerWhenDowngrade , "same-origin" => Policy :: SameOrigin , "origin" => Policy :: Origin , "origin-when-cross-origin" => Policy :: OriginWhenCrossOrigin , "strict-origin" => Policy :: StrictOrigin , "strict-origin-when-cross-origin" => Policy :: StrictOriginWhenCrossOrigin , "unsafe-url" | "always" => Policy :: UnsafeUrl , _ => continue , }) ; } known . ok_or_else (Error :: invalid) } }
};
}
