macro_rules! parse_markdown {
    () => {
        # [cfg (not (feature = "unstable-markdown"))] fn parse_markdown (lines : & [String]) -> (String , Option < String >) { if lines . iter () . any (| s | is_blank (s)) { let paragraphs = split_paragraphs (lines) ; let short = paragraphs [0] . clone () ; let long = paragraphs . join ("\n\n") ; (short , Some (long)) } else { let short = merge_lines (lines) ; (short , None) } }
    };
}

parse_markdown!();