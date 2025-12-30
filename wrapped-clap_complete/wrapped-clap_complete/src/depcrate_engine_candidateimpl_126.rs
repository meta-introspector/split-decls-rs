// Generated macro for impl_126 (impl)
macro_rules! Depcrate_engine_candidateimpl_126 {
() => {
// Module: crate::engine::candidate
// Provides: {"impl_126"}
// Dependencies: {}
# [doc = " Reflection API"] impl CompletionCandidate { # [doc = " Get the literal value being proposed for completion"] pub fn get_value (& self) -> & OsStr { & self . value } # [doc = " Get the help message of the completion candidate"] pub fn get_help (& self) -> Option < & StyledStr > { self . help . as_ref () } # [doc = " Get the id used for de-duplicating"] pub fn get_id (& self) -> Option < & String > { self . id . as_ref () } # [doc = " Get the grouping tag"] pub fn get_tag (& self) -> Option < & StyledStr > { self . tag . as_ref () } # [doc = " Get the grouping tag"] pub fn get_display_order (& self) -> Option < usize > { self . display_order } # [doc = " Get the visibility of the completion candidate"] pub fn is_hide_set (& self) -> bool { self . hidden } }
};
}
