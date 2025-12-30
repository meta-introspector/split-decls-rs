// Generated macro for impl_1006 (impl)
macro_rules! Depcrate_themeimpl_1006 {
() => {
// Module: crate::theme
// Provides: {"impl_1006"}
// Dependencies: {}
# [rustfmt :: skip] impl render :: GitColours for Theme { fn not_modified (& self) -> Style { self . ui . punctuation () } # [allow (clippy :: new_ret_no_self)] fn new (& self) -> Style { self . ui . git . unwrap_or_default () . new () } fn modified (& self) -> Style { self . ui . git . unwrap_or_default () . modified () } fn deleted (& self) -> Style { self . ui . git . unwrap_or_default () . deleted () } fn renamed (& self) -> Style { self . ui . git . unwrap_or_default () . renamed () } fn type_change (& self) -> Style { self . ui . git . unwrap_or_default () . typechange () } fn ignored (& self) -> Style { self . ui . git . unwrap_or_default () . ignored () } fn conflicted (& self) -> Style { self . ui . git . unwrap_or_default () . conflicted () } }
};
}
