macro_rules! deps {
    () => {
        SliceDrain!();
    };
}

macro_rules! impl_1362 {
    () => {
        deps!();
        impl < 'data , T : 'data > DoubleEndedIterator for SliceDrain < 'data , T > { fn next_back (& mut self) -> Option < Self :: Item > { let ptr : * const T = self . iter . next_back () ? ; Some (unsafe { ptr :: read (ptr) }) } }
    };
}

impl_1362!();