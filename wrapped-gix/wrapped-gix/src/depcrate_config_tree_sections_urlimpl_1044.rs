// Generated macro for impl_1044 (impl)
macro_rules! Depcrate_config_tree_sections_urlimpl_1044 {
() => {
// Module: crate::config::tree::sections::url
// Provides: {"impl_1044"}
// Dependencies: {}
impl Url { # [doc = " The `url.<base>.insteadOf` key"] pub const INSTEAD_OF : keys :: Any = keys :: Any :: new ("insteadOf" , & config :: Tree :: URL) . with_subsection_requirement (BASE_PARAMETER) ; # [doc = " The `url.<base>.pushInsteadOf` key"] pub const PUSH_INSTEAD_OF : keys :: Any = keys :: Any :: new ("pushInsteadOf" , & config :: Tree :: URL) . with_subsection_requirement (BASE_PARAMETER) ; }
};
}
