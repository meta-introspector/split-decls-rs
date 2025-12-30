// Generated macro for draw_overflow (function)
macro_rules! Depcrate_render_tui_draw_progressdraw_overflow {
() => {
// Module: crate::render::tui::draw::progress
// Provides: {"draw_overflow"}
// Dependencies: {}
pub fn draw_overflow (entries : & [(Key , Task)] , buf : & mut Buffer , bound : Rect , label_offset : u16 , num_entries_on_display : u16 , offset : u16 ,) { let (count , mut progress_fraction) = entries . iter () . take (offset as usize) . chain (entries . iter () . skip ((offset + num_entries_on_display) as usize)) . fold ((0usize , 0f32) , | (count , progress_fraction) , (_key , value) | { let progress = value . progress . as_ref () . and_then (| p | p . fraction ()) . unwrap_or_default () ; (count + 1 , progress_fraction + progress) }) ; progress_fraction /= count as f32 ; let label = format ! ("{} …{} skipped and {} more" , if label_offset == 0 { "" } else { VERTICAL_LINE } , offset , entries . len () . saturating_sub ((offset + num_entries_on_display + 1) as usize)) ; let (progress_rect , style) = draw_progress_bar_fn (buf , bound , progress_fraction , | _ | Color :: Green) ; let bg_color = Color :: Red ; fill_background (rect :: offset_x (bound , progress_rect . right () - 1) , buf , bg_color) ; let color_text_according_to_progress = move | _g : & str , x : u16 , _y : u16 | { if x < progress_rect . right () { style } else { style . bg (bg_color) } } ; draw_text_nowrap_fn (rect :: offset_x (bound , label_offset) , buf , label , color_text_according_to_progress ,) ; let help_text = "⇊ = d|↓ = j|⇈ = u|↑ = k " ; draw_text_nowrap_fn (rect :: snap_to_right (bound , block_width (help_text)) , buf , help_text , color_text_according_to_progress ,) ; }
};
}
