macro_rules! deps {
    () => {
        StrippedStr!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 's > Iterator for StrippedStr < 's > { type Item = & 's str ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_str (& mut self . bytes , & mut self . state) } }
    };
}

impl_4!();