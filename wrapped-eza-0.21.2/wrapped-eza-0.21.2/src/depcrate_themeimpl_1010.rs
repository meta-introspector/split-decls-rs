// Generated macro for impl_1010 (impl)
macro_rules! Depcrate_themeimpl_1010 {
() => {
// Module: crate::theme
// Provides: {"impl_1010"}
// Dependencies: {}
# [rustfmt :: skip] impl render :: PermissionsColours for Theme { fn dash (& self) -> Style { self . ui . punctuation () } fn user_read (& self) -> Style { self . ui . perms . unwrap_or_default () . user_read () } fn user_write (& self) -> Style { self . ui . perms . unwrap_or_default () . user_write () } fn user_execute_file (& self) -> Style { self . ui . perms . unwrap_or_default () . user_execute_file () } fn user_execute_other (& self) -> Style { self . ui . perms . unwrap_or_default () . user_execute_other () } fn group_read (& self) -> Style { self . ui . perms . unwrap_or_default () . group_read () } fn group_write (& self) -> Style { self . ui . perms . unwrap_or_default () . group_write () } fn group_execute (& self) -> Style { self . ui . perms . unwrap_or_default () . group_execute () } fn other_read (& self) -> Style { self . ui . perms . unwrap_or_default () . other_read () } fn other_write (& self) -> Style { self . ui . perms . unwrap_or_default () . other_write () } fn other_execute (& self) -> Style { self . ui . perms . unwrap_or_default () . other_execute () } fn special_user_file (& self) -> Style { self . ui . perms . unwrap_or_default () . special_user_file () } fn special_other (& self) -> Style { self . ui . perms . unwrap_or_default () . special_other () } fn attribute (& self) -> Style { self . ui . perms . unwrap_or_default () . attribute () } }
};
}
