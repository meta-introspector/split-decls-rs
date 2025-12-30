// Generated macro for text (function)
macro_rules! Depcrate_destroytext {
() => {
// Module: crate::destroy
// Provides: {"text"}
// Dependencies: {}
# [doc = " draw some text fading in and out from black to red and back"] # [expect (clippy :: cast_possible_truncation , clippy :: cast_precision_loss)] fn text (frame_count : usize , area : Rect , buf : & mut Buffer) { let sub_frame = frame_count . saturating_sub (TEXT_DELAY) ; if sub_frame == 0 { return ; } let logo = indoc :: indoc ! { "
        ██████      ████    ██████    ████    ██████  ██    ██  ██
        ██    ██  ██    ██    ██    ██    ██    ██    ██    ██  ██
        ██████    ████████    ██    ████████    ██    ██    ██  ██
        ██  ██    ██    ██    ██    ██    ██    ██    ██    ██  ██
        ██    ██  ██    ██    ██    ██    ██    ██      ████    ██
    " } ; let logo_text = Text :: styled (logo , Color :: Rgb (255 , 255 , 255)) ; let area = centered_rect (area , logo_text . width () as u16 , logo_text . height () as u16) ; let mask_buf = & mut Buffer :: empty (area) ; logo_text . render (area , mask_buf) ; let percentage = (sub_frame as f64 / 480.0) . clamp (0.0 , 1.0) ; for row in area . rows () { for col in row . columns () { let cell = & mut buf [(col . x , col . y)] ; let mask_cell = & mut mask_buf [(col . x , col . y)] ; cell . set_symbol (mask_cell . symbol ()) ; let cell_color = cell . style () . bg . unwrap_or (Color :: Rgb (0 , 0 , 0)) ; let mask_color = mask_cell . style () . fg . unwrap_or (Color :: Rgb (255 , 0 , 0)) ; let color = blend (mask_color , cell_color , percentage) ; cell . set_style (Style :: new () . fg (color)) ; } } }
};
}
