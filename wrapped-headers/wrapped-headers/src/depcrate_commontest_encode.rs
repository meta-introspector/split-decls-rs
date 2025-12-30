// Generated macro for test_encode (function)
macro_rules! Depcrate_commontest_encode {
() => {
// Module: crate::common
// Provides: {"test_encode"}
// Dependencies: {}
# [cfg (test)] fn test_encode < T : crate :: Header > (header : T) -> :: http :: HeaderMap { use crate :: HeaderMapExt ; let mut map = :: http :: HeaderMap :: new () ; map . typed_insert (header) ; map }
};
}
