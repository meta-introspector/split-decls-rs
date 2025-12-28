macro_rules! deps {
    () => {
        ValuesMut!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for ValuesMut < 'a , K , V > { type Item = & 'a mut V ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| bucket | & mut bucket . value) } }
    };
}

impl_130!()