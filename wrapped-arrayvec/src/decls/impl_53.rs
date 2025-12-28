macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < T , const CAP : usize > DoubleEndedIterator for IntoIter < T , CAP > { fn next_back (& mut self) -> Option < Self :: Item > { if self . index == self . v . len () { None } else { unsafe { let new_len = self . v . len () - 1 ; self . v . set_len (new_len) ; Some (ptr :: read (self . v . get_unchecked_ptr (new_len))) } } } }
    };
}

impl_53!()