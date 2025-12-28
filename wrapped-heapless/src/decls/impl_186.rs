macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (k , v) | (k as & K , v)) } }
    };
}

impl_186!();