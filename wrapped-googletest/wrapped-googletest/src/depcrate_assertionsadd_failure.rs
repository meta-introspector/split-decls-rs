// Generated macro for add_failure (macro)
macro_rules! Depcrate_assertionsadd_failure {
() => {
// Module: crate::assertions
// Provides: {"add_failure"}
// Dependencies: {}
# [doc = " Generates a failure marking the test as failed but continue execution."] # [doc = ""] # [doc = " This is a **not-fatal** failure. The test continues execution even after the"] # [doc = " macro execution."] # [doc = ""] # [doc = " This can only be invoked inside tests with the"] # [doc = " [`gtest`][crate::gtest] attribute. The failure must be generated"] # [doc = " in the same thread as that running the test itself."] # [doc = ""] # [doc = " ```ignore"] # [doc = " use googletest::prelude::*;"] # [doc = ""] # [doc = " #[gtest]"] # [doc = " fn should_fail_but_not_abort() {"] # [doc = "     add_failure!();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " One may include formatted arguments in the failure message:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " use googletest::prelude::*;"] # [doc = ""] # [doc = " #[gtest]"] # [doc = " fn should_fail_but_not_abort() {"] # [doc = "     add_failure!(\"I am just a fake test: {}\", \"a fake test indeed\");"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! add_failure { ($ ($ message : expr) ,+ $ (,) ?) => { { $ crate :: GoogleTestSupport :: and_log_failure ($ crate :: assertions :: internal :: create_fail_result (format ! ($ ($ message) ,*) ,)) ; } } ; () => { add_failure ! ("Failed") } ; }
};
}
