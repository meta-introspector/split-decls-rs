macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < const CAP : usize > Deref for ArrayString < CAP > { type Target = str ; # [inline] fn deref (& self) -> & str { unsafe { let sl = slice :: from_raw_parts (self . as_ptr () , self . len ()) ; str :: from_utf8_unchecked (sl) } } }
    };
}

impl_9!()