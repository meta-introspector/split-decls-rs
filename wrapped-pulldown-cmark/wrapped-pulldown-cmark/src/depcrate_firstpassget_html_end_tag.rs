// Generated macro for get_html_end_tag (function)
macro_rules! Depcrate_firstpassget_html_end_tag {
() => {
// Module: crate::firstpass
// Provides: {"get_html_end_tag"}
// Dependencies: {}
# [doc = " Assumes `text_bytes` is preceded by `<`."] fn get_html_end_tag (text_bytes : & [u8]) -> Option < & 'static str > { static BEGIN_TAGS : & [& [u8] ; 4] = & [b"pre" , b"style" , b"script" , b"textarea"] ; static ST_BEGIN_TAGS : & [& [u8] ; 3] = & [b"!--" , b"?" , b"![CDATA["] ; for (beg_tag , end_tag) in BEGIN_TAGS . iter () . zip (["</pre>" , "</style>" , "</script>" , "</textarea>"] . iter ()) { let tag_len = beg_tag . len () ; if text_bytes . len () < tag_len { break ; } if ! text_bytes [.. tag_len] . eq_ignore_ascii_case (beg_tag) { continue ; } if text_bytes . len () == tag_len { return Some (end_tag) ; } let s = text_bytes [tag_len] ; if is_ascii_whitespace (s) || s == b'>' { return Some (end_tag) ; } } for (beg_tag , end_tag) in ST_BEGIN_TAGS . iter () . zip (["-->" , "?>" , "]]>"] . iter ()) { if text_bytes . starts_with (beg_tag) { return Some (end_tag) ; } } if text_bytes . len () > 1 && text_bytes [0] == b'!' && text_bytes [1] . is_ascii_alphabetic () { Some (">") } else { None } }
};
}
