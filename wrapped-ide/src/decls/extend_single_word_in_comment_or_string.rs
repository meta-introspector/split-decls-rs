macro_rules! extend_single_word_in_comment_or_string {
    () => {
        fn extend_single_word_in_comment_or_string (leaf : & SyntaxToken , offset : TextSize ,) -> Option < TextRange > { let text : & str = leaf . text () ; let cursor_position : u32 = (offset - leaf . text_range () . start ()) . into () ; let (before , after) = text . split_at (cursor_position as usize) ; fn non_word_char (c : char) -> bool { ! (c . is_alphanumeric () || c == '_') } let start_idx = before . rfind (non_word_char) ? as u32 ; let end_idx = after . find (non_word_char) . unwrap_or (after . len ()) as u32 ; fn ceil_char_boundary (text : & str , index : u32) -> u32 { (index ..) . find (| & index | text . is_char_boundary (index as usize)) . unwrap_or (text . len () as u32) } let from : TextSize = ceil_char_boundary (text , start_idx + 1) . into () ; let to : TextSize = (cursor_position + end_idx) . into () ; let range = TextRange :: new (from , to) ; if range . is_empty () { None } else { Some (range + leaf . text_range () . start ()) } }
    };
}

extend_single_word_in_comment_or_string!()