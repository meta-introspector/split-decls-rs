macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < K , V , const N : usize > Iterator for IntoIter < K , V , N > { type Item = (K , V) ; fn next (& mut self) -> Option < Self :: Item > { self . entries . pop () . map (| bucket | (bucket . key , bucket . value)) } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
    };
}

impl_114!();