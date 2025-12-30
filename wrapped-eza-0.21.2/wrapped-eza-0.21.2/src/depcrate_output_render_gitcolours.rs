// Generated macro for Colours (trait)
macro_rules! Depcrate_output_render_gitColours {
() => {
// Module: crate::output::render::git
// Provides: {"Colours"}
// Dependencies: {}
pub trait Colours { fn not_modified (& self) -> Style ; # [allow (clippy :: new_ret_no_self , clippy :: wrong_self_convention)] fn new (& self) -> Style ; fn modified (& self) -> Style ; fn deleted (& self) -> Style ; fn renamed (& self) -> Style ; fn type_change (& self) -> Style ; fn ignored (& self) -> Style ; fn conflicted (& self) -> Style ; }
};
}
