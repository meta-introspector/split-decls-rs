macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; # [inline] fn next (& mut self) -> Option < (& 'a K , & 'a mut V) > { if self . remaining == 0 { None } else { self . remaining -= 1 ; unsafe { let head = self . head . as_ptr () ; let (key , value) = (* head) . entry_mut () ; self . head = Some ((* head) . links . value . next) ; Some ((key , value)) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
    };
}

impl_72!()