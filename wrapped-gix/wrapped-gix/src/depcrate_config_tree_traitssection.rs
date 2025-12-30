// Generated macro for Section (trait)
macro_rules! Depcrate_config_tree_traitsSection {
() => {
// Module: crate::config::tree::traits
// Provides: {"Section"}
// Dependencies: {}
# [doc = " Provide information about a configuration section."] pub trait Section { # [doc = " The section name, like `remote` in `remote.origin.url`."] fn name (& self) -> & str ; # [doc = " The keys directly underneath it for carrying configuration values."] fn keys (& self) -> & [& dyn Key] ; # [doc = " The list of sub-section names, which may be empty if there are no statically known sub-sections."] fn sub_sections (& self) -> & [& dyn Section] { & [] } # [doc = " The parent section if this is a statically known sub-section."] fn parent (& self) -> Option < & dyn Section > { None } }
};
}
