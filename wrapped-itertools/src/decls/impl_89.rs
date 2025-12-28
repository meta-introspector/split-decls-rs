macro_rules! deps {
    () => {
        Batching!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < B , F , I > Iterator for Batching < I , F > where I : Iterator , F : FnMut (& mut I) -> Option < B > , { type Item = B ; # [inline] fn next (& mut self) -> Option < Self :: Item > { (self . f) (& mut self . iter) } }
    };
}

impl_89!()