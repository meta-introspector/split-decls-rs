// Generated macro for tests (module)
macro_rules! Depcrate_jsonttests {
() => {
// Module: crate::jsont
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn base64_basic () { let b64 = | s : & str | base64_standard (s . as_bytes ()) ; assert_eq ! (b64 ("") , "") ; assert_eq ! (b64 ("f") , "Zg==") ; assert_eq ! (b64 ("fo") , "Zm8=") ; assert_eq ! (b64 ("foo") , "Zm9v") ; assert_eq ! (b64 ("foob") , "Zm9vYg==") ; assert_eq ! (b64 ("fooba") , "Zm9vYmE=") ; assert_eq ! (b64 ("foobar") , "Zm9vYmFy") ; } }
};
}
