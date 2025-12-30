// Generated macro for impl_933 (impl)
macro_rules! Depcrate_config_tree_sections_gitoxideimpl_933 {
() => {
// Module: crate::config::tree::sections::gitoxide
// Provides: {"impl_933"}
// Dependencies: {}
impl Section for Gitoxide { fn name (& self) -> & str { "gitoxide" } fn keys (& self) -> & [& dyn Key] { & [& Self :: USER_AGENT , & Self :: TRACE_PACKET , & Self :: PARSE_PRECIOUS] } fn sub_sections (& self) -> & [& dyn Section] { & [& Self :: ALLOW , & Self :: AUTHOR , & Self :: CORE , & Self :: COMMIT , & Self :: COMMITTER , & Self :: CREDENTIALS , & Self :: HTTP , & Self :: HTTPS , & Self :: OBJECTS , & Self :: SSH , & Self :: USER , & Self :: PATHSPEC ,] } }
};
}
