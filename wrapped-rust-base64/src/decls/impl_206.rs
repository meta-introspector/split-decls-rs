macro_rules! deps {
    () => {
        ParseAlphabetError!();
        Alphabet!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl convert :: TryFrom < & str > for Alphabet { type Error = ParseAlphabetError ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Self :: new (value) } }
    };
}

impl_206!()