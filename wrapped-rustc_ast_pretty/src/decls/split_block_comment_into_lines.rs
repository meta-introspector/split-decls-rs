macro_rules! split_block_comment_into_lines {
    () => {
        fn split_block_comment_into_lines (text : & str , col : CharPos) -> Vec < String > { let mut res : Vec < String > = vec ! [] ; let mut lines = text . lines () ; res . extend (lines . next () . map (| it | it . to_string ())) ; for line in lines { res . push (trim_whitespace_prefix (line , col) . to_string ()) } res }
    };
}

split_block_comment_into_lines!();