macro_rules! format_doc_comment {
    () => {
        pub (crate) fn format_doc_comment (lines : & [String] , preprocess : bool , force_long : bool ,) -> (Option < String > , Option < String >) { if preprocess { let (short , long) = parse_markdown (lines) ; let long = long . or_else (| | force_long . then (| | short . clone ())) ; (Some (remove_period (short)) , long) } else if let Some (first_blank) = lines . iter () . position (| s | is_blank (s)) { let short = lines [.. first_blank] . join ("\n") ; let long = lines . join ("\n") ; (Some (short) , Some (long)) } else { let short = lines . join ("\n") ; let long = force_long . then (| | short . clone ()) ; (Some (short) , long) } }
    };
}

format_doc_comment!();