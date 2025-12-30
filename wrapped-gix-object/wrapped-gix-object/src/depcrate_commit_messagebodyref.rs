// Generated macro for BodyRef (struct)
macro_rules! Depcrate_commit_messageBodyRef {
() => {
// Module: crate::commit::message
// Provides: {"BodyRef"}
// Dependencies: {}
# [doc = " A reference to a message body, further parsed to only contain the non-trailer parts."] # [doc = ""] # [doc = " See [git-interpret-trailers](https://git-scm.com/docs/git-interpret-trailers) for more information"] # [doc = " on what constitutes trailers and not that this implementation is only good for typical sign-off footer or key-value parsing."] # [doc = ""] # [doc = " Note that we only parse trailers from the bottom of the body."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub struct BodyRef < 'a > { body_without_trailer : & 'a BStr , start_of_trailer : & 'a [u8] , }
};
}
