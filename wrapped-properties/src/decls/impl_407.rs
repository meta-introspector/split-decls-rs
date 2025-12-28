macro_rules! deps {
    () => {
        ScriptWithExt!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl TrieValue for ScriptWithExt { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u16 :: try_from (i) . map (Self) } fn to_u32 (self) -> u32 { u32 :: from (self . 0) } }
    };
}

impl_407!();