// Generated macro for level_to_style (function)
macro_rules! Depcrate_render_tui_draw_messageslevel_to_style {
() => {
// Module: crate::render::tui::draw::messages
// Provides: {"level_to_style"}
// Dependencies: {}
fn level_to_style (level : MessageLevel) -> Style { use MessageLevel :: * ; Style :: default () . fg (Color :: Black) . add_modifier (Modifier :: BOLD) . bg (match level { Info => Color :: White , Failure => Color :: Red , Success => Color :: Green , }) }
};
}
