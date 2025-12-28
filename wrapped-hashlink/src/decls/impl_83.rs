macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < K , V > Drop for Drain < '_ , K , V > { # [inline] fn drop (& mut self) { for _ in 0 .. self . remaining { unsafe { let mut tail = NonNull :: new_unchecked (self . tail . as_ptr ()) ; self . tail = Some (tail . as_ref () . links . value . prev) ; tail . as_mut () . take_entry () ; push_free (& mut * self . free . as_ptr () , tail) ; } } } }
    };
}

impl_83!()