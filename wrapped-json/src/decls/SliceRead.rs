macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! SliceRead {
    () => {
        deps!();
        # [doc = " JSON input source that reads from a slice of bytes."] pub struct SliceRead < 'a > { slice : & 'a [u8] , # [doc = " Index of the *next* byte that will be returned by next() or peek()."] index : usize , # [cfg (feature = "raw_value")] raw_buffering_start_index : usize , }
    };
}

SliceRead!();