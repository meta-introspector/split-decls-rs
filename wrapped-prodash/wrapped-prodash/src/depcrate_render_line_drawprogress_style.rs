// Generated macro for progress_style (function)
macro_rules! Depcrate_render_line_drawprogress_style {
() => {
// Module: crate::render::line::draw
// Provides: {"progress_style"}
// Dependencies: {}
fn progress_style (p : & Value) -> Style { use crate :: progress :: State :: * ; match p . state { Running => if let Some (fraction) = p . fraction () { if fraction > 0.8 { Color :: Green } else { Color :: Yellow } } else { Color :: White } . normal () , Halted (_ , _) => Color :: Red . dimmed () , Blocked (_ , _) => Color :: Red . normal () , } }
};
}
