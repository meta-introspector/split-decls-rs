macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < 'a , K , V > DoubleEndedIterator for Keys < 'a , K , V > { # [inline] fn next_back (& mut self) -> Option < & 'a K > { self . inner . next_back () . map (| e | e . 0) } }
    };
}

impl_90!();