macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < const CAP : usize > DerefMut for ArrayString < CAP > { # [inline] fn deref_mut (& mut self) -> & mut str { unsafe { let len = self . len () ; let sl = slice :: from_raw_parts_mut (self . as_mut_ptr () , len) ; str :: from_utf8_unchecked_mut (sl) } } }
    };
}

impl_10!();