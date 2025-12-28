macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T , const CAP : usize > Iterator for IntoIter < T , CAP > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . index == self . v . len () { None } else { unsafe { let index = self . index ; self . index = index + 1 ; Some (ptr :: read (self . v . get_unchecked_ptr (index))) } } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . v . len () - self . index ; (len , Some (len)) } }
    };
}

impl_52!()