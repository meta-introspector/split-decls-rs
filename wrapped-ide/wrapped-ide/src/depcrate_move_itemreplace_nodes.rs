// Generated macro for replace_nodes (function)
macro_rules! Depcrate_move_itemreplace_nodes {
() => {
// Module: crate::move_item
// Provides: {"replace_nodes"}
// Dependencies: {}
fn replace_nodes < 'a > (range : TextRange , mut first : & 'a SyntaxNode , mut second : & 'a SyntaxNode ,) -> TextEdit { let cursor_offset = if range . is_empty () { if first . text_range () . contains_range (range) { Some (range . start () - first . text_range () . start ()) } else if second . text_range () . contains_range (range) { mem :: swap (& mut first , & mut second) ; Some (range . start () - first . text_range () . start ()) } else { None } } else { None } ; let first_with_cursor = match cursor_offset { Some (offset) => { let mut item_text = first . text () . to_string () ; item_text . insert_str (offset . into () , "$0") ; item_text } None => first . text () . to_string () , } ; let mut edit = TextEditBuilder :: default () ; diff (first , second) . into_text_edit (& mut edit) ; edit . replace (second . text_range () , first_with_cursor) ; edit . finish () }
};
}
