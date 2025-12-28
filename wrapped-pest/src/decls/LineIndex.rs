macro_rules! LineIndex {
    () => {
        # [derive (Clone)] pub struct LineIndex { # [doc = " Offset (bytes) the the beginning of each line, zero-based"] line_offsets : Vec < usize > , }
    };
}

LineIndex!();