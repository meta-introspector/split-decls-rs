macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        # [cfg (feature = "unicode")] impl < 'a > Iterator for Fields < 'a > { type Item = & 'a [u8] ; # [inline] fn next (& mut self) -> Option < & 'a [u8] > { self . it . next () } }
    };
}

impl_84!()