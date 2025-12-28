macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for Keys < 'a , K , V > { type Item = & 'a K ; # [inline] fn next (& mut self) -> Option < & 'a K > { self . inner . next () . map (| e | e . 0) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_89!()