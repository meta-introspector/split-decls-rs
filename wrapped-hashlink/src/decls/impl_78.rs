macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < K , V > DoubleEndedIterator for Drain < '_ , K , V > { # [inline] fn next_back (& mut self) -> Option < (K , V) > { if self . remaining == 0 { return None ; } self . remaining -= 1 ; unsafe { let mut tail = NonNull :: new_unchecked (self . tail . as_ptr ()) ; self . tail = Some (tail . as_ref () . links . value . prev) ; let entry = tail . as_mut () . take_entry () ; push_free (& mut * self . free . as_ptr () , tail) ; Some (entry) } } }
    };
}

impl_78!();