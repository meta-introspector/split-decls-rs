macro_rules! split_paragraphs {
    () => {
        # [cfg (not (feature = "unstable-markdown"))] fn split_paragraphs (lines : & [String]) -> Vec < String > { use std :: iter ; let mut last_line = 0 ; iter :: from_fn (| | { let slice = & lines [last_line ..] ; let start = slice . iter () . position (| s | ! is_blank (s)) . unwrap_or (0) ; let slice = & slice [start ..] ; let len = slice . iter () . position (| s | is_blank (s)) . unwrap_or (slice . len ()) ; last_line += start + len ; if len != 0 { Some (merge_lines (& slice [.. len])) } else { None } }) . collect () }
    };
}

split_paragraphs!();