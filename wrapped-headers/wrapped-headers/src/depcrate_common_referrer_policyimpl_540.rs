// Generated macro for impl_540 (impl)
macro_rules! Depcrate_common_referrer_policyimpl_540 {
() => {
// Module: crate::common::referrer_policy
// Provides: {"impl_540"}
// Dependencies: {}
impl < 'a > From < & 'a Policy > for HeaderValue { fn from (policy : & 'a Policy) -> HeaderValue { HeaderValue :: from_static (match * policy { Policy :: NoReferrer => "no-referrer" , Policy :: NoReferrerWhenDowngrade => "no-referrer-when-downgrade" , Policy :: SameOrigin => "same-origin" , Policy :: Origin => "origin" , Policy :: OriginWhenCrossOrigin => "origin-when-cross-origin" , Policy :: StrictOrigin => "strict-origin" , Policy :: StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin" , Policy :: UnsafeUrl => "unsafe-url" , }) } }
};
}
