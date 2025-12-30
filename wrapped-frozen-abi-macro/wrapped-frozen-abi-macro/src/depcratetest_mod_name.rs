// Generated macro for test_mod_name (function)
macro_rules! Depcratetest_mod_name {
() => {
// Module: crate
// Provides: {"test_mod_name"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] fn test_mod_name (type_name : & Ident) -> Ident { Ident :: new (& format ! ("{type_name}_frozen_abi") , Span :: call_site ()) }
};
}
