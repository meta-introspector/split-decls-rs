macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < K > DoubleEndedIterator for IntoIter < K > { # [inline] fn next_back (& mut self) -> Option < K > { self . iter . next_back () . map (| (k , _) | k) } }
    };
}

impl_165!();