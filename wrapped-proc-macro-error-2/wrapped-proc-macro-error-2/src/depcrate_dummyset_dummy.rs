// Generated macro for set_dummy (function)
macro_rules! Depcrate_dummyset_dummy {
() => {
// Module: crate::dummy
// Provides: {"set_dummy"}
// Dependencies: {}
# [doc = " Sets dummy token stream which will be appended to `compile_error!(msg);...`"] # [doc = " invocations in case you'll emit any errors."] # [doc = ""] # [doc = " See [guide](../index.html#guide)."] # [allow (clippy :: must_use_candidate)] pub fn set_dummy (dummy : TokenStream) -> Option < TokenStream > { check_correctness () ; DUMMY_IMPL . with (| old_dummy | old_dummy . replace (Some (dummy))) }
};
}
