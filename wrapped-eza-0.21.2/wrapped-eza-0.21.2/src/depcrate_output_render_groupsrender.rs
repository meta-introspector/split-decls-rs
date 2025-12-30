// Generated macro for Render (trait)
macro_rules! Depcrate_output_render_groupsRender {
() => {
// Module: crate::output::render::groups
// Provides: {"Render"}
// Dependencies: {}
pub trait Render { fn render < C : Colours , U : Users + Groups > (self , colours : & C , users : & U , user_format : UserFormat , group_format : GroupFormat , file_user : Option < User > ,) -> TextCell ; }
};
}
