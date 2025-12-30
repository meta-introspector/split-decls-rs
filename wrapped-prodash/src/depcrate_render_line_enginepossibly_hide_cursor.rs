// Generated macro for possibly_hide_cursor (function)
macro_rules! Depcrate_render_line_enginepossibly_hide_cursor {
() => {
// Module: crate::render::line::engine
// Provides: {"possibly_hide_cursor"}
// Dependencies: {}
# [allow (unused_mut)] fn possibly_hide_cursor (out : & mut impl io :: Write , mut hide_cursor : bool) -> bool { if hide_cursor { crosstermion :: execute ! (out , crosstermion :: cursor :: Hide) . is_ok () } else { false } }
};
}
