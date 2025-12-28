macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < K , V , const N : usize > Iterator for IntoIter < K , V , N > where K : Eq , { type Item = (K , V) ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
    };
}

impl_178!();