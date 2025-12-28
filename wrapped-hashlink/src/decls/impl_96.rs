macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'a , K , V > DoubleEndedIterator for Values < 'a , K , V > { # [inline] fn next_back (& mut self) -> Option < & 'a V > { self . inner . next_back () . map (| e | e . 1) } }
    };
}

impl_96!()