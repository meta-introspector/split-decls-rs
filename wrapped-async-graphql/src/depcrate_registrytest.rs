// Generated macro for test (module)
macro_rules! Depcrate_registrytest {
() => {
// Module: crate::registry
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: registry :: MetaDirectiveInvocation ; # [test] fn test_directive_invocation_dsl () { let expected = r#"@testDirective(int_value: 1, str_value: "abc")"# ; assert_eq ! (expected . to_string () , MetaDirectiveInvocation { name : "testDirective" . to_string () , args : [("int_value" . to_string () , 1u32 . into ()) , ("str_value" . to_string () , "abc" . into ())] . into () , } . sdl ()) } }
};
}
