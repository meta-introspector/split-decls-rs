macro_rules! deps {
    () => {
        FixedSizeListIter!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'a , T > Iterator for FixedSizeListIter < 'a , T > { type Item = (usize , & 'a T) ; fn next (& mut self) -> Option < Self :: Item > { if self . len > 0 { let front = self . front ; let node = self . list . node_ref (front) . unwrap () ; self . front = node . next ; self . len -= 1 ; Some ((front , & node . data)) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_9!();