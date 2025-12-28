macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < K , V > Iterator for Drain < '_ , K , V > { type Item = (K , V) ; # [inline] fn next (& mut self) -> Option < (K , V) > { if self . remaining == 0 { return None ; } self . remaining -= 1 ; unsafe { let mut head = NonNull :: new_unchecked (self . head . as_ptr ()) ; self . head = Some (head . as_ref () . links . value . next) ; let entry = head . as_mut () . take_entry () ; push_free (self . free . as_mut () , head) ; Some (entry) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . remaining , Some (self . remaining)) } }
    };
}

impl_74!()