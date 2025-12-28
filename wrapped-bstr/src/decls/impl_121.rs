macro_rules! deps {
    () => {
        DrainBytes!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'a > Iterator for DrainBytes < 'a > { type Item = u8 ; # [inline] fn next (& mut self) -> Option < u8 > { self . it . next () } }
    };
}

impl_121!()