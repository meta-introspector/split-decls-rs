// Generated macro for transform (function)
macro_rules! Depcratetransform {
() => {
// Module: crate
// Provides: {"transform"}
// Dependencies: {}
pub fn transform (s : & str , flipped : bool , elongate : bool) -> Cow < '_ , str > { let (small_map , caps_map) = if flipped { (FLIPPED_SMALL_MAP , FLIPPED_CAPS_MAP) } else { (TRANSFORM_SMALL_MAP , TRANSFORM_CAPS_MAP) } ; RE_AZ . replace_all (s , | caps : & Captures | { let ch = caps [0] . chars () . next () . unwrap () ; let cc = ch as u8 ; if (97 ..= 122) . contains (& cc) { let pos = cc - 97 ; let new_char = small_map [pos as usize] ; if elongate && (cc == 97 || cc == 101 || cc == 111 || cc == 117) { let mut s = new_char . to_string () ; s . push (new_char) ; s } else { new_char . to_string () } } else if (65 ..= 90) . contains (& cc) { let pos = cc - 65 ; let new_char = caps_map [pos as usize] ; new_char . to_string () } else { ch . to_string () } }) }
};
}
