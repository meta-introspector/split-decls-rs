macro_rules! deps {
    () => {
        OwningIter!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < K : Eq + Hash > Iterator for OwningIter < K > { type Item = K ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (k , _) | k) } }
    };
}

impl_19!();