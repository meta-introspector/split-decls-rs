// Generated macro for is_word_byte (function)
macro_rules! Depcrate_utf8is_word_byte {
() => {
// Module: crate::utf8
// Provides: {"is_word_byte"}
// Dependencies: {}
# [doc = " Returns true if and only if the given byte is considered a word character."] # [doc = " This only applies to ASCII."] pub (crate) fn is_word_byte (b : u8) -> bool { const fn mkwordset () -> [bool ; 256] { let mut set = [false ; 256] ; set [b'_' as usize] = true ; let mut byte = b'0' ; while byte <= b'9' { set [byte as usize] = true ; byte += 1 ; } byte = b'A' ; while byte <= b'Z' { set [byte as usize] = true ; byte += 1 ; } byte = b'a' ; while byte <= b'z' { set [byte as usize] = true ; byte += 1 ; } set } const WORD : [bool ; 256] = mkwordset () ; WORD [b as usize] }
};
}
