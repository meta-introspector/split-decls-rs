macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| bucket | (& bucket . key , & mut bucket . value)) } }
    };
}

impl_124!();