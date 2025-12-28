macro_rules! deps {
    () => {
        LinesWithTerminator!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for LinesWithTerminator < 'a > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { let end = self . bytes . len () . checked_sub (1) ? ; match self . bytes [.. end] . rfind_byte (b'\n') { None => { let line = self . bytes ; self . bytes = b"" ; Some (line) } Some (end) => { let line = & self . bytes [end + 1 ..] ; self . bytes = & self . bytes [..= end] ; Some (line) } } } }
    };
}

impl_108!();