// Generated macro for impl_1007 (impl)
macro_rules! Depcrate_themeimpl_1007 {
() => {
// Module: crate::theme
// Provides: {"impl_1007"}
// Dependencies: {}
# [rustfmt :: skip] impl render :: GitRepoColours for Theme { fn branch_main (& self) -> Style { self . ui . git_repo . unwrap_or_default () . branch_main () } fn branch_other (& self) -> Style { self . ui . git_repo . unwrap_or_default () . branch_other () } fn no_repo (& self) -> Style { self . ui . punctuation () } fn git_clean (& self) -> Style { self . ui . git_repo . unwrap_or_default () . git_clean () } fn git_dirty (& self) -> Style { self . ui . git_repo . unwrap_or_default () . git_dirty () } }
};
}
