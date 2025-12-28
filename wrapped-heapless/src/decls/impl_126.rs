macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for Keys < 'a , K , V > { type Item = & 'a K ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| bucket | & bucket . key) } }
    };
}

impl_126!()