// Generated macro for draw_progress_bar (function)
macro_rules! Depcrate_render_line_drawdraw_progress_bar {
() => {
// Module: crate::render::line::draw
// Provides: {"draw_progress_bar"}
// Dependencies: {}
fn draw_progress_bar (p : & Value , style : Style , mut blocks_available : u16 , colored : bool , buf : & mut Vec < ANSIString < '_ > >) { let mut brush = color :: Brush :: new (colored) ; let styled_brush = brush . style (style) ; blocks_available = blocks_available . saturating_sub (3) ; buf . push (" [" . into ()) ; match p . fraction () { Some (mut fraction) => { fraction = fraction . min (1.0) ; blocks_available = blocks_available . saturating_sub (1) ; let progress_blocks = (blocks_available as f32 * fraction) . floor () as usize ; buf . push (styled_brush . paint (format ! ("{:=<width$}" , "" , width = progress_blocks))) ; buf . push (styled_brush . paint (">")) ; buf . push (styled_brush . style (style . dimmed ()) . paint (format ! ("{:-<width$}" , "" , width = (blocks_available - progress_blocks as u16) as usize))) ; } None => { const CHARS : [char ; 6] = ['=' , '=' , '=' , ' ' , ' ' , ' '] ; buf . push (styled_brush . paint ((p . step . load (Ordering :: SeqCst) .. usize :: MAX) . take (blocks_available as usize) . map (| idx | CHARS [idx % CHARS . len ()]) . rev () . collect :: < String > () ,) ,) ; } } buf . push ("]" . into ()) ; }
};
}
