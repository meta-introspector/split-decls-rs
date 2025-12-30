// Generated macro for test_decode (function)
macro_rules! Depcrate_commontest_decode {
() => {
// Module: crate::common
// Provides: {"test_decode"}
// Dependencies: {}
# [cfg (test)] fn test_decode < T : crate :: Header > (values : & [& str]) -> Option < T > { use crate :: HeaderMapExt ; let mut map = :: http :: HeaderMap :: new () ; for val in values { map . append (T :: name () , val . parse () . unwrap ()) ; } map . typed_get () }
};
}
