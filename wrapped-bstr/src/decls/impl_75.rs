macro_rules! deps {
    () => {
        FindReverse!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'h , 'n > Iterator for FindReverse < 'h , 'n > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { self . it . next () } }
    };
}

impl_75!();