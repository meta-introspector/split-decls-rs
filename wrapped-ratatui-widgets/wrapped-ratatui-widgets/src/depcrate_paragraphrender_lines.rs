// Generated macro for render_lines (function)
macro_rules! Depcrate_paragraphrender_lines {
() => {
// Module: crate::paragraph
// Provides: {"render_lines"}
// Dependencies: {}
fn render_lines < 'a , C : LineComposer < 'a > > (mut composer : C , area : Rect , buf : & mut Buffer) { let mut y = 0 ; while let Some (ref wrapped) = composer . next_line () { render_line (wrapped , area , buf , y) ; y += 1 ; if y >= area . height { break ; } } }
};
}
