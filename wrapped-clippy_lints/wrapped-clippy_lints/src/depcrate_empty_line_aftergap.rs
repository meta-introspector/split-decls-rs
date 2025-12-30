// Generated macro for Gap (struct)
macro_rules! Depcrate_empty_line_afterGap {
() => {
// Module: crate::empty_line_after
// Provides: {"Gap"}
// Dependencies: {}
# [doc = " Represents a set of attrs/doc comments separated by 1 or more empty lines"] # [doc = ""] # [doc = " ```ignore"] # [doc = " /// chunk 1 docs"] # [doc = " // not an empty line so also part of chunk 1"] # [doc = " #[chunk_1_attrs] // <-- prev_stop"] # [doc = ""] # [doc = " /* gap */"] # [doc = ""] # [doc = " /// chunk 2 docs // <-- next_stop"] # [doc = " #[chunk_2_attrs]"] # [doc = " ```"] struct Gap < 'a > { # [doc = " The span of individual empty lines including the newline at the end of the line"] empty_lines : Vec < Span > , has_comment : bool , next_stop : & 'a Stop , prev_stop : & 'a Stop , # [doc = " The chunk that includes [`prev_stop`](Self::prev_stop)"] prev_chunk : & 'a [Stop] , }
};
}
