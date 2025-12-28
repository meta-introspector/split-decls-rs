macro_rules! deps {
    () => {
        LinesWithTerminator!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < 'a > Iterator for LinesWithTerminator < 'a > { type Item = & 'a [u8] ; # [inline] fn next (& mut self) -> Option < & 'a [u8] > { match self . bytes . find_byte (b'\n') { None if self . bytes . is_empty () => None , None => { let line = self . bytes ; self . bytes = b"" ; Some (line) } Some (end) => { let line = & self . bytes [..= end] ; self . bytes = & self . bytes [end + 1 ..] ; Some (line) } } } }
    };
}

impl_107!()