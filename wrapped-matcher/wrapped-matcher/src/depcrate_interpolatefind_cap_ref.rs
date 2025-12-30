// Generated macro for find_cap_ref (function)
macro_rules! Depcrate_interpolatefind_cap_ref {
() => {
// Module: crate::interpolate
// Provides: {"find_cap_ref"}
// Dependencies: {}
# [doc = " Parses a possible reference to a capture group name in the given text,"] # [doc = " starting at the beginning of `replacement`."] # [doc = ""] # [doc = " If no such valid reference could be found, None is returned."] # [inline] fn find_cap_ref (replacement : & [u8]) -> Option < CaptureRef < '_ > > { let mut i = 0 ; if replacement . len () <= 1 || replacement [0] != b'$' { return None ; } let mut brace = false ; i += 1 ; if replacement [i] == b'{' { brace = true ; i += 1 ; } let mut cap_end = i ; while replacement . get (cap_end) . map_or (false , is_valid_cap_letter) { cap_end += 1 ; } if cap_end == i { return None ; } let cap = std :: str :: from_utf8 (& replacement [i .. cap_end]) . expect ("valid UTF-8 capture name") ; if brace { if ! replacement . get (cap_end) . map_or (false , | & b | b == b'}') { return None ; } cap_end += 1 ; } Some (CaptureRef { cap : match cap . parse :: < u32 > () { Ok (i) => Ref :: Number (i as usize) , Err (_) => Ref :: Named (cap) , } , end : cap_end , }) }
};
}
