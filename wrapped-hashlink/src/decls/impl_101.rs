macro_rules! deps {
    () => {
        ValuesMut!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 'a , K , V > DoubleEndedIterator for ValuesMut < 'a , K , V > { # [inline] fn next_back (& mut self) -> Option < & 'a mut V > { self . inner . next_back () . map (| e | e . 1) } }
    };
}

impl_101!()