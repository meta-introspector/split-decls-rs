macro_rules! deps {
    () => {
        Lines!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'a > Iterator for Lines < 'a > { type Item = & 'a [u8] ; # [inline] fn next (& mut self) -> Option < & 'a [u8] > { Some (trim_last_terminator (self . it . next () ?)) } }
    };
}

impl_102!()