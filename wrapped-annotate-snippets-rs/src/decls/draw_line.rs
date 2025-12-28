macro_rules! deps {
    () => {
        Renderer!();
        Margin!();
        ElementStyle!();
        StyledBuffer!();
    };
}

macro_rules! draw_line {
    () => {
        deps!();
        # [allow (clippy :: too_many_arguments)] fn draw_line (renderer : & Renderer , buffer : & mut StyledBuffer , source_string : & str , line_index : usize , line_offset : usize , width_offset : usize , code_offset : usize , max_line_num_len : usize , margin : Margin ,) -> usize { debug_assert ! (! source_string . contains ('\t')) ; let line_len = str_width (source_string) ; let mut left = margin . left (line_len) ; let right = margin . right (line_len) ; let mut taken = 0 ; let mut skipped = 0 ; let code : String = source_string . chars () . skip_while (| ch | { let w = char_width (* ch) ; if skipped < left { skipped += w ; true } else { false } }) . take_while (| ch | { taken += char_width (* ch) ; taken <= (right - left) }) . collect () ; if skipped > left { left += skipped - left ; } let placeholder = renderer . decor_style . margin () ; let padding = str_width (placeholder) ; let (width_taken , bytes_taken) = if margin . was_cut_left () { let mut bytes_taken = 0 ; let mut width_taken = 0 ; for ch in code . chars () { width_taken += char_width (ch) ; bytes_taken += ch . len_utf8 () ; if width_taken >= padding { break ; } } buffer . puts (line_offset , code_offset , placeholder , ElementStyle :: LineNumber ,) ; (width_taken , bytes_taken) } else { (0 , 0) } ; buffer . puts (line_offset , code_offset + width_taken , & code [bytes_taken ..] , ElementStyle :: Quotation ,) ; if line_len > right { let mut char_taken = 0 ; let mut width_taken_inner = 0 ; for ch in code . chars () . rev () { width_taken_inner += char_width (ch) ; char_taken += 1 ; if width_taken_inner >= padding { break ; } } buffer . puts (line_offset , code_offset + width_taken + code [bytes_taken ..] . chars () . count () - char_taken , placeholder , ElementStyle :: LineNumber ,) ; } buffer . puts (line_offset , 0 , & maybe_anonymized (renderer , line_index , max_line_num_len) , ElementStyle :: LineNumber ,) ; draw_col_separator_no_space (renderer , buffer , line_offset , width_offset - 2) ; left }
    };
}

draw_line!()