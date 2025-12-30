// Generated macro for find_cap_ref (function)
macro_rules! Depcrate_expandfind_cap_ref {
() => {
// Module: crate::expand
// Provides: {"find_cap_ref"}
// Dependencies: {}
fn find_cap_ref (mut replacement : & [u8]) -> Option < CaptureRef > { if replacement . len () <= 1 || replacement [0] != b'$' { return None ; } let mut brace = false ; replacement = & replacement [1 ..] ; if replacement [0] == b'{' { brace = true ; replacement = & replacement [1 ..] ; } let mut cap_end = 0 ; while replacement . get (cap_end) . map_or (false , is_valid_cap_letter) { cap_end += 1 ; } if cap_end == 0 { return None ; } let cap = str :: from_utf8 (& replacement [.. cap_end]) . ok () . expect ("valid UTF-8 capture name") ; if brace { if ! replacement . get (cap_end) . map_or (false , | & b | b == b'}') { return None ; } cap_end += 1 ; } Some (CaptureRef { rest : & replacement [cap_end ..] , cap : match cap . parse :: < u32 > () { Ok (i) => Ref :: Number (i as usize) , Err (_) => Ref :: Named (cap) , } , }) }
};
}
