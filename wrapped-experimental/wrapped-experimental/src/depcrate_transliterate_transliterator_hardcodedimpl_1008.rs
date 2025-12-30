// Generated macro for impl_1008 (impl)
macro_rules! Depcrate_transliterate_transliterator_hardcodedimpl_1008 {
() => {
// Module: crate::transliterate::transliterator::hardcoded
// Provides: {"impl_1008"}
// Dependencies: {}
impl HexTransliterator { pub (super) fn new (prefix : & 'static str , suffix : & 'static str , min_length : u8 , case : Case ,) -> Self { Self { prefix , suffix , min_length , case , } } pub (super) fn transliterate (& self , mut rep : Replaceable) { while ! rep . is_finished () { let mut matcher = rep . start_match () ; let c = Utf8Matcher :: < Forward > :: next_char (& matcher) ; let c = c . unwrap () ; Utf8Matcher :: < Forward > :: match_and_consume_char (& mut matcher , c) ; let mut dest = matcher . finish_match () ; let c_u32 = c as u32 ; let length = (u32 :: BITS - c_u32 . leading_zeros ()) . div_ceil (4) ; let padding = self . min_length . saturating_sub (length as u8) ; dest . apply_size_hint (self . prefix . len () + padding as usize + length as usize + self . suffix . len () ,) ; dest . push_str (self . prefix) ; for _ in 0 .. padding { dest . push_str ("0") ; } let mut remaining_c = c_u32 ; let mut buf = [0 ; 6] ; for slot in buf . iter_mut () { if c_u32 == 0 { break ; } * slot = match remaining_c & 0xF { x @ 0x0 ..= 0x9 => b'0' + x as u8 , x @ 0xA ..= 0xF if self . case == Case :: Lower => b'a' + (x - 0xA) as u8 , x => b'A' + (x - 0xA) as u8 , } ; remaining_c >>= 4 ; } for c in buf [.. length as usize] . iter () . rev () { dest . push (* c as char) ; } dest . push_str (self . suffix) ; } } }
};
}
