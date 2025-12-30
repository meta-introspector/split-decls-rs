// Generated macro for macro_8860 (macro)
macro_rules! Depcrate_pub_usemacro_8860 {
() => {
// Module: crate::pub_use
// Provides: {"macro_8860"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Restricts the usage of `pub use ...`"] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " A project may wish to limit `pub use` instances to prevent"] # [doc = " unintentional exports, or to encourage placing exported items directly in public modules."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " pub mod outer {"] # [doc = "     mod inner {"] # [doc = "         pub struct Test {}"] # [doc = "     }"] # [doc = "     pub use inner::Test;"] # [doc = " }"] # [doc = ""] # [doc = " use outer::Test;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " pub mod outer {"] # [doc = "     pub struct Test {}"] # [doc = " }"] # [doc = ""] # [doc = " use outer::Test;"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub PUB_USE , restriction , "restricts the usage of `pub use`" }
};
}
