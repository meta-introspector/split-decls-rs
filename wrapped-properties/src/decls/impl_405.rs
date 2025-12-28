macro_rules! deps {
    () => {
        Script!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        impl TrieValue for Script { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u16 :: try_from (i) . map (Script) } fn to_u32 (self) -> u32 { u32 :: from (self . 0) } }
    };
}

impl_405!()