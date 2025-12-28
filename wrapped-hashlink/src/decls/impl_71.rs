macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; # [inline] fn next (& mut self) -> Option < (& 'a K , & 'a V) > { if self . remaining == 0 { None } else { self . remaining -= 1 ; unsafe { let (key , value) = (* self . head) . entry_ref () ; self . head = (* self . head) . links . value . next . as_ptr () ; Some ((key , value)) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
    };
}

impl_71!();