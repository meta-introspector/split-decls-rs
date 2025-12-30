// Generated macro for find_cap_ref (function)
macro_rules! Depcrate_interpolatefind_cap_ref {
() => {
// Module: crate::interpolate
// Provides: {"find_cap_ref"}
// Dependencies: {}
# [doc = " Parses a possible reference to a capture group name in the given text,"] # [doc = " starting at the beginning of `replacement`."] # [doc = ""] # [doc = " If no such valid reference could be found, None is returned."] # [doc = ""] # [doc = " Note that this returns a \"possible\" reference because this routine doesn't"] # [doc = " know whether the reference is to a valid group or not. If it winds up not"] # [doc = " being a valid reference, then it should be replaced with the empty string."] fn find_cap_ref (replacement : & [u8]) -> Option < CaptureRef < '_ > > { let mut i = 0 ; let rep : & [u8] = replacement ; if rep . len () <= 1 || rep [0] != b'$' { return None ; } i += 1 ; if rep [i] == b'{' { return find_cap_ref_braced (rep , i + 1) ; } let mut cap_end = i ; while rep . get (cap_end) . copied () . map_or (false , is_valid_cap_letter) { cap_end += 1 ; } if cap_end == i { return None ; } let cap = core :: str :: from_utf8 (& rep [i .. cap_end]) . expect ("valid UTF-8 capture name") ; Some (CaptureRef { cap : match cap . parse :: < usize > () { Ok (i) => Ref :: Number (i) , Err (_) => Ref :: Named (cap) , } , end : cap_end , }) }
};
}
