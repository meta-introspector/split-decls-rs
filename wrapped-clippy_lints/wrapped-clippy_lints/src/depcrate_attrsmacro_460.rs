// Generated macro for macro_460 (macro)
macro_rules! Depcrate_attrsmacro_460 {
() => {
// Module: crate::attrs
// Provides: {"macro_460"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `#[cfg_attr(clippy, allow(clippy::lint))]`"] # [doc = " and suggests to replace it with `#[allow(clippy::lint)]`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " There is no reason to put clippy attributes behind a clippy `cfg` as they are not"] # [doc = " run by anything else than clippy."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #![cfg_attr(clippy, allow(clippy::deprecated_cfg_attr))]"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #![allow(clippy::deprecated_cfg_attr)]"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub UNNECESSARY_CLIPPY_CFG , suspicious , "usage of `cfg_attr(clippy, allow(clippy::lint))` instead of `allow(clippy::lint)`" }
};
}
