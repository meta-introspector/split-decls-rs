macro_rules! deps {
    () => {
        Find!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'h , 'n > Iterator for Find < 'h , 'n > { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { self . it . next () } }
    };
}

impl_72!()