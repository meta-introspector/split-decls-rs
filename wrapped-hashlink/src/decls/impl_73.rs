macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < K , V > Iterator for IntoIter < K , V > { type Item = (K , V) ; # [inline] fn next (& mut self) -> Option < (K , V) > { if self . remaining == 0 { return None ; } self . remaining -= 1 ; unsafe { let head = self . head . as_ptr () ; self . head = Some ((* head) . links . value . next) ; let mut e = Box :: from_raw (head) ; Some (e . take_entry ()) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
    };
}

impl_73!()