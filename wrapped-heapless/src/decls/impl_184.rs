macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (k , v) | (k , v)) } }
    };
}

impl_184!()