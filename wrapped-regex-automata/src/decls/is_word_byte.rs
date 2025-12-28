macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! is_word_byte {
    () => {
        deps!();
        # [doc = " Returns true if and only if the given byte is considered a word character."] # [doc = " This only applies to ASCII."] # [doc = ""] # [doc = " This was copied from regex-syntax so that we can use it to determine the"] # [doc = " starting DFA state while searching without depending on regex-syntax. The"] # [doc = " definition is never going to change, so there's no maintenance/bit-rot"] # [doc = " hazard here."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn is_word_byte (b : u8) -> bool { const fn mkwordset () -> [bool ; 256] { let mut set = [false ; 256] ; set [b'_' as usize] = true ; let mut byte = b'0' ; while byte <= b'9' { set [byte as usize] = true ; byte += 1 ; } byte = b'A' ; while byte <= b'Z' { set [byte as usize] = true ; byte += 1 ; } byte = b'a' ; while byte <= b'z' { set [byte as usize] = true ; byte += 1 ; } set } const WORD : [bool ; 256] = mkwordset () ; WORD [b as usize] }
    };
}

is_word_byte!();