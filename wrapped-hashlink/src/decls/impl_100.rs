macro_rules! deps {
    () => {
        ValuesMut!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for ValuesMut < 'a , K , V > { type Item = & 'a mut V ; # [inline] fn next (& mut self) -> Option < & 'a mut V > { self . inner . next () . map (| e | e . 1) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_100!();