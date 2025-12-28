macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < T , const N : usize > Iterator for IntoIter < T , N > { type Item = T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . begin == self . end . value (Self :: is_zst ()) { None } else { unsafe { let ptr = self . as_mut_ptr () ; let value = ptr . add (self . begin) . read () ; self . begin += 1 ; Some (value) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let size = self . end . value (Self :: is_zst ()) - self . begin ; (size , Some (size)) } }
    };
}

impl_107!()