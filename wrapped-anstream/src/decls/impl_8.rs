macro_rules! deps {
    () => {
        StripStrIter!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 's > Iterator for StripStrIter < 's > { type Item = & 's str ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_str (& mut self . bytes , self . state) } }
    };
}

impl_8!()