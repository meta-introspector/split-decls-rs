// Generated macro for layout_multiline_text (function)
macro_rules! Depcrate_element_textlayout_multiline_text {
() => {
// Module: crate::element::text
// Provides: {"layout_multiline_text"}
// Dependencies: {}
fn layout_multiline_text < 'a , F : FnMut (& 'a str) > (text : & 'a str , max_width : u32 , font : FontDesc < 'a > , mut func : F ,) { for line in text . lines () { if max_width == 0 || line . is_empty () { func (line) ; } else { let mut indices = line . char_indices () . map (| (idx , _) | idx) . peekable () ; let it = std :: iter :: from_fn (| | { let start_idx = match indices . next () { Some (idx) => idx , None => return None , } ; for idx in indices . by_ref () { let substring = & line [start_idx .. idx] ; let width = font . box_size (substring) . unwrap_or ((0 , 0)) . 0 as i32 ; if width > max_width as i32 { break ; } } let end_idx = match indices . peek () { Some (idx) => * idx , None => line . bytes () . len () , } ; Some (& line [start_idx .. end_idx]) }) ; for chunk in it { func (chunk) ; } } } }
};
}
