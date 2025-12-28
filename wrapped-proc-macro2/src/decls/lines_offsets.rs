macro_rules! lines_offsets {
    () => {
        # [doc = " Computes the offsets of each line in the given source string"] # [doc = " and the total number of characters"] # [cfg (all (span_locations , not (fuzzing)))] fn lines_offsets (s : & str) -> (usize , Vec < usize >) { let mut lines = vec ! [0] ; let mut total = 0 ; for ch in s . chars () { total += 1 ; if ch == '\n' { lines . push (total) ; } } (total , lines) }
    };
}

lines_offsets!();