// Generated macro for count_header_cols (function)
macro_rules! Depcrate_firstpasscount_header_cols {
() => {
// Module: crate::firstpass
// Provides: {"count_header_cols"}
// Dependencies: {}
# [doc = " Computes the number of header columns in a table line by computing the number of dividing pipes"] # [doc = " that aren't followed or preceded by whitespace."] fn count_header_cols (bytes : & [u8] , mut pipes : usize , mut start : usize , last_pipe_ix : usize ,) -> usize { start += scan_whitespace_no_nl (& bytes [start ..]) ; if bytes [start] == b'|' { pipes -= 1 ; } if scan_blank_line (& bytes [(last_pipe_ix + 1) ..]) . is_some () { pipes } else { pipes + 1 } }
};
}
