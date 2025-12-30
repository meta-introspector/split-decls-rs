// Generated macro for scan_link_label (function)
macro_rules! Depcrate_parsescan_link_label {
() => {
// Module: crate::parse
// Provides: {"scan_link_label"}
// Dependencies: {}
# [doc = " Scans an inline link label, which cannot be interrupted."] # [doc = " Returns number of bytes (including brackets) and label on success."] fn scan_link_label < 'text > (tree : & Tree < Item > , text : & 'text str , options : Options ,) -> Option < (usize , ReferenceLabel < 'text >) > { let bytes = text . as_bytes () ; if bytes . len () < 2 || bytes [0] != b'[' { return None ; } let linebreak_handler = | bytes : & [u8] | Some (skip_container_prefixes (tree , bytes , options)) ; if options . contains (Options :: ENABLE_FOOTNOTES) && b'^' == bytes [1] && bytes . get (2) != Some (& b']') { let linebreak_handler : & dyn Fn (& [u8]) -> Option < usize > = if options . has_gfm_footnotes () { & | _ | None } else { & linebreak_handler } ; if let Some ((byte_index , cow)) = scan_link_label_rest (& text [2 ..] , linebreak_handler , tree . is_in_table ()) { return Some ((byte_index + 2 , ReferenceLabel :: Footnote (cow))) ; } } let (byte_index , cow) = scan_link_label_rest (& text [1 ..] , & linebreak_handler , tree . is_in_table ()) ? ; Some ((byte_index + 1 , ReferenceLabel :: Link (cow))) }
};
}
