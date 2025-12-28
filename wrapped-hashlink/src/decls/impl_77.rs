macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < K , V > DoubleEndedIterator for IntoIter < K , V > { # [inline] fn next_back (& mut self) -> Option < (K , V) > { if self . remaining == 0 { return None ; } self . remaining -= 1 ; unsafe { let mut e = * Box :: from_raw (self . tail . as_ptr ()) ; self . tail = Some (e . links . value . prev) ; Some (e . take_entry ()) } } }
    };
}

impl_77!();