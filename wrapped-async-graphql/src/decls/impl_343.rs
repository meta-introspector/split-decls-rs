macro_rules! deps {
    () => {
        QueryPathNode!();
        Parents!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < 'a > Iterator for Parents < 'a > { type Item = & 'a QueryPathNode < 'a > ; fn next (& mut self) -> Option < Self :: Item > { let parent = self . 0 . parent ; if let Some (parent) = parent { self . 0 = parent ; } parent } }
    };
}

impl_343!()