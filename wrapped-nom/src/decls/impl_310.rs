macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        impl < 'a > Offset for & 'a str { fn offset (& self , second : & Self) -> usize { let fst = self . as_ptr () ; let snd = second . as_ptr () ; snd as usize - fst as usize } }
    };
}

impl_310!();