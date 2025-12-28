macro_rules! deps {
    () => {
        Score!();
        Token!();
        Indents!();
        IndentLevel!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Indents { fn at_token (tokens : & [Token] , token_idx : usize , indent_of_token : impl Fn (Token) -> IndentLevel ,) -> Indents { let (leading_blank_lines , indent_previous_line) = tokens [.. token_idx] . iter () . rev () . enumerate () . find_map (| (i , & token) | { if i == MAX_BLANKS { Some ((i , IndentLevel (0))) } else { let level = indent_of_token (token) ; if level == IndentLevel :: BLANK { None } else { Some ((i , level)) } } }) . unwrap_or ((token_idx , IndentLevel :: BLANK)) ; let at_eof = token_idx == tokens . len () ; let (trailing_blank_lines , indent_next_line) = if at_eof { (0 , IndentLevel :: BLANK) } else { tokens [token_idx + 1 ..] . iter () . enumerate () . find_map (| (i , & token) | { if i == MAX_BLANKS { Some ((i , IndentLevel (0))) } else { let level = indent_of_token (token) ; if level == IndentLevel :: BLANK { None } else { Some ((i , level)) } } }) . unwrap_or ((token_idx , IndentLevel :: BLANK)) } ; let indent = tokens . get (token_idx) . map_or (IndentLevel :: BLANK , | & token | indent_of_token (token)) ; Indents { indent , prev_indent : indent_previous_line , next_indent : indent_next_line , leading_blanks : leading_blank_lines as u8 , trailing_blanks : trailing_blank_lines as u8 , } } fn score (& self) -> Score { let mut penalty = 0 ; if self . prev_indent == IndentLevel :: BLANK && self . leading_blanks == 0 { penalty += START_OF_FILE_PENALTY ; } if self . next_indent == IndentLevel :: BLANK && self . trailing_blanks == 0 { penalty += END_OF_FILE_PENALTY ; } let trailing_blank_lines = if self . indent == IndentLevel :: BLANK { self . trailing_blanks as i32 + 1 } else { 0 } ; let total_blank_lines = trailing_blank_lines + self . leading_blanks as i32 ; penalty += TOTAL_BLANK_LINE_WEIGHT * total_blank_lines + trailing_blank_lines * TRAILING_BLANK_LINES_WEIGHT ; let indent = self . indent . or (self . next_indent) ; if indent != IndentLevel :: BLANK && self . prev_indent != IndentLevel :: BLANK { match indent . 0 . cmp (& self . prev_indent . 0) { Ordering :: Equal => { } Ordering :: Less if self . next_indent . 0 <= indent . 0 => { penalty += if total_blank_lines != 0 { RELATIVE_DEDENT_WITH_BLANK_PENALTY } else { RELATIVE_DEDENT_PENALTY } } Ordering :: Less => { penalty += if total_blank_lines != 0 { RELATIVE_OUTDENT_WITH_BLANK_PENALTY } else { RELATIVE_OUTDENT_PENALTY } } Ordering :: Greater => { penalty += if total_blank_lines != 0 { RELATIVE_INDENT_WITH_BLANK_PENALTY } else { RELATIVE_INDENT_PENALTY } } } } Score { indent : indent . map_or (- 1 , i32 :: from) , penalty , } } }
    };
}

impl_45!();