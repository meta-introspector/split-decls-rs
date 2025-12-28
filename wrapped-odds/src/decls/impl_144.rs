macro_rules! deps {
    () => {
        CharStr!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Deref for CharStr { type Target = str ; fn deref (& self) -> & str { unsafe { str :: from_utf8_unchecked (& self . buf [.. self . len as usize]) } } }
    };
}

impl_144!()