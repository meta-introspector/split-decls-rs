macro_rules! deps {
    () => {
        SliceDrain!();
    };
}

macro_rules! impl_1361 {
    () => {
        deps!();
        impl < 'data , T : 'data > Iterator for SliceDrain < 'data , T > { type Item = T ; fn next (& mut self) -> Option < T > { let ptr : * const T = self . iter . next () ? ; Some (unsafe { ptr :: read (ptr) }) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } fn count (self) -> usize { self . iter . len () } }
    };
}

impl_1361!()