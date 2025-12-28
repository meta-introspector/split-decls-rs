macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'a , K , V > DoubleEndedIterator for IterMut < 'a , K , V > { # [inline] fn next_back (& mut self) -> Option < (& 'a K , & 'a mut V) > { if self . remaining == 0 { None } else { self . remaining -= 1 ; unsafe { let tail = self . tail . as_ptr () ; self . tail = Some ((* tail) . links . value . prev) ; let (key , value) = (* tail) . entry_mut () ; Some ((key , value)) } } } }
    };
}

impl_76!();