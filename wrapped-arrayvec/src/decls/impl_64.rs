macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'a , T : 'a , const CAP : usize > Drop for Drain < 'a , T , CAP > { fn drop (& mut self) { while let Some (_) = self . next () { } if self . tail_len > 0 { unsafe { let source_vec = & mut * self . vec ; let start = source_vec . len () ; let tail = self . tail_start ; let ptr = source_vec . as_mut_ptr () ; ptr :: copy (ptr . add (tail) , ptr . add (start) , self . tail_len) ; source_vec . set_len (start + self . tail_len) ; } } } }
    };
}

impl_64!()