macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'a , K , V > DoubleEndedIterator for Iter < 'a , K , V > { # [inline] fn next_back (& mut self) -> Option < (& 'a K , & 'a V) > { if self . remaining == 0 { None } else { self . remaining -= 1 ; unsafe { let tail = self . tail ; self . tail = (* tail) . links . value . prev . as_ptr () ; let (key , value) = (* tail) . entry_ref () ; Some ((key , value)) } } } }
    };
}

impl_75!();