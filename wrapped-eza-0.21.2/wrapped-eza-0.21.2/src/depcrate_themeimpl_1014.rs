// Generated macro for impl_1014 (impl)
macro_rules! Depcrate_themeimpl_1014 {
() => {
// Module: crate::theme
// Provides: {"impl_1014"}
// Dependencies: {}
# [rustfmt :: skip] impl render :: SecurityCtxColours for Theme { fn none (& self) -> Style { self . ui . security_context . unwrap_or_default () . none () } fn selinux_colon (& self) -> Style { self . ui . security_context . unwrap_or_default () . selinux () . colon () } fn selinux_user (& self) -> Style { self . ui . security_context . unwrap_or_default () . selinux () . user () } fn selinux_role (& self) -> Style { self . ui . security_context . unwrap_or_default () . selinux () . role () } fn selinux_type (& self) -> Style { self . ui . security_context . unwrap_or_default () . selinux () . typ () } fn selinux_range (& self) -> Style { self . ui . security_context . unwrap_or_default () . selinux () . range () } }
};
}
