macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for Values < 'a , K , V > { type Item = & 'a V ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| bucket | & bucket . value) } }
    };
}

impl_128!()