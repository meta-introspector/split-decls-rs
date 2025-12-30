// Generated macro for macro_490 (macro)
macro_rules! Depcrate_await_holding_invalidmacro_490 {
() => {
// Module: crate::await_holding_invalid
// Provides: {"macro_490"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Allows users to configure types which should not be held across await"] # [doc = " suspension points."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " There are some types which are perfectly safe to use concurrently from"] # [doc = " a memory access perspective, but that will cause bugs at runtime if"] # [doc = " they are held in such a way."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```toml"] # [doc = " await-holding-invalid-types = ["] # [doc = "   # You can specify a type name"] # [doc = "   \"CustomLockType\","] # [doc = "   # You can (optionally) specify a reason"] # [doc = "   { path = \"OtherCustomLockType\", reason = \"Relies on a thread local\" }"] # [doc = " ]"] # [doc = " ```"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # async fn baz() {}"] # [doc = " struct CustomLockType;"] # [doc = " struct OtherCustomLockType;"] # [doc = " async fn foo() {"] # [doc = "   let _x = CustomLockType;"] # [doc = "   let _y = OtherCustomLockType;"] # [doc = "   baz().await; // Lint violation"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub AWAIT_HOLDING_INVALID_TYPE , suspicious , "holding a type across an await point which is not allowed to be held as per the configuration" }
};
}
