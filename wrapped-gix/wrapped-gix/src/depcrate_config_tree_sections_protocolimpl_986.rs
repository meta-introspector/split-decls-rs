// Generated macro for impl_986 (impl)
macro_rules! Depcrate_config_tree_sections_protocolimpl_986 {
() => {
// Module: crate::config::tree::sections::protocol
// Provides: {"impl_986"}
// Dependencies: {}
impl Protocol { # [doc = " The `protocol.allow` key."] pub const ALLOW : Allow = Allow :: new_with_validate ("allow" , & config :: Tree :: PROTOCOL , validate :: Allow) ; # [doc = " The `protocol.version` key."] pub const VERSION : Version = Version :: new_with_validate ("version" , & config :: Tree :: PROTOCOL , validate :: Version) ; # [doc = " The `protocol.<name>` subsection"] pub const NAME_PARAMETER : NameParameter = NameParameter ; }
};
}
