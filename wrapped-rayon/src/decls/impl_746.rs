macro_rules! deps {
    () => {
        PanicFuseIter!();
    };
}

macro_rules! impl_746 {
    () => {
        deps!();
        impl < 'a , I > Iterator for PanicFuseIter < 'a , I > where I : Iterator , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { if self . fuse . panicked () { None } else { self . base . next () } } fn size_hint (& self) -> (usize , Option < usize >) { self . base . size_hint () } }
    };
}

impl_746!()