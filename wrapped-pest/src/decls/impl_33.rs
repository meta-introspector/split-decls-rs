macro_rules! deps {
    () => {
        LineIndex!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl LineIndex { pub fn new (text : & str) -> LineIndex { let mut line_offsets : Vec < usize > = alloc :: vec ! [0] ; let mut offset = 0 ; for c in text . chars () { offset += c . len_utf8 () ; if c == '\n' { line_offsets . push (offset) ; } } LineIndex { line_offsets } } # [doc = " Returns (line, col) of pos."] # [doc = ""] # [doc = " The pos is a byte offset, start from 0, e.g. \"ab\" is 2, \"你好\" is 6"] pub fn line_col (& self , input : & str , pos : usize) -> (usize , usize) { let line = self . line_offsets . partition_point (| & it | it <= pos) - 1 ; let first_offset = self . line_offsets [line] ; let line_str = & input [first_offset .. pos] ; let col = line_str . chars () . count () ; (line + 1 , col + 1) } }
    };
}

impl_33!()