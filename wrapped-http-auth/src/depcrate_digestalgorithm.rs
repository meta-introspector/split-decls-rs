// Generated macro for Algorithm (enum)
macro_rules! Depcrate_digestAlgorithm {
() => {
// Module: crate::digest
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " Supported algorithm from the [HTTP Digest Algorithm Values"] # [doc = " registry](https://www.iana.org/assignments/http-dig-alg/http-dig-alg.xhtml)."] # [doc = ""] # [doc = " This doesn't store whether the session variant (`<Algorithm>-sess`) was"] # [doc = " requested; see [`DigestClient::session`] for that."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Algorithm { Md5 , Sha256 , Sha512Trunc256 , }
};
}
