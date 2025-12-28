macro_rules! block_string_value {
    () => {
        pub (super) fn block_string_value (raw : & str) -> String { let lines : Vec < _ > = raw . split ("\r\n") . flat_map (| s | s . split (['\r' , '\n'] . as_ref ())) . collect () ; let common_indent = lines . iter () . skip (1) . copied () . filter_map (| line | line . find (| c | c != '\t' && c != ' ')) . min () . unwrap_or (0) ; let line_has_content = | line : & str | line . as_bytes () . iter () . any (| & c | c != b'\t' && c != b' ') ; let first_contentful_line = lines . iter () . copied () . position (line_has_content) . unwrap_or (lines . len ()) ; let ending_lines_start = lines . iter () . copied () . rposition (line_has_content) . map_or (0 , | i | i + 1) ; lines . iter () . copied () . enumerate () . take (ending_lines_start) . skip (first_contentful_line) . map (| (i , line) | { if i != 0 && line . len () >= common_indent { & line [common_indent ..] } else { line } }) . enumerate () . flat_map (| (i , line) | { if i == 0 { [] . as_ref () } else { ['\n'] . as_ref () } . iter () . copied () . chain (line . chars ()) }) . collect () }
    };
}

block_string_value!();