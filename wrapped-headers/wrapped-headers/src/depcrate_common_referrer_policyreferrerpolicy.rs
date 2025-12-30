// Generated macro for ReferrerPolicy (struct)
macro_rules! Depcrate_common_referrer_policyReferrerPolicy {
() => {
// Module: crate::common::referrer_policy
// Provides: {"ReferrerPolicy"}
// Dependencies: {}
# [doc = " `Referrer-Policy` header, part of"] # [doc = " [Referrer Policy](https://www.w3.org/TR/referrer-policy/#referrer-policy-header)"] # [doc = ""] # [doc = " The `Referrer-Policy` HTTP header specifies the referrer"] # [doc = " policy that the user agent applies when determining what"] # [doc = " referrer information should be included with requests made,"] # [doc = " and with browsing contexts created from the context of the"] # [doc = " protected resource."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Referrer-Policy: 1#policy-token"] # [doc = " policy-token   = \"no-referrer\" / \"no-referrer-when-downgrade\""] # [doc = "                  / \"same-origin\" / \"origin\""] # [doc = "                  / \"origin-when-cross-origin\" / \"unsafe-url\""] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `no-referrer`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::ReferrerPolicy;"] # [doc = ""] # [doc = " let rp = ReferrerPolicy::NO_REFERRER;"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct ReferrerPolicy (Policy) ;
};
}
