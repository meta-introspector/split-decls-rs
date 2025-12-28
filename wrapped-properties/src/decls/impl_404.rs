macro_rules! impl_404 {
    () => {
        impl TrieValue for GeneralCategory { type TryFromU32Error = & 'static str ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { GeneralCategory :: new_from_u8 (i . try_into () . unwrap_or (u8 :: MAX)) . ok_or ("Cannot parse GeneralCategory from integer") } fn to_u32 (self) -> u32 { u32 :: from (self as u8) } }
    };
}

impl_404!()