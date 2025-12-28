macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for Values < 'a , K , V > { type Item = & 'a V ; # [inline] fn next (& mut self) -> Option < & 'a V > { self . inner . next () . map (| e | e . 1) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_95!();