macro_rules! deps {
    () => {
        GraphemeClusterBreak!();
    };
}

macro_rules! impl_410 {
    () => {
        deps!();
        impl TrieValue for GraphemeClusterBreak { type TryFromU32Error = TryFromIntError ; fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > { u8 :: try_from (i) . map (Self) } fn to_u32 (self) -> u32 { u32 :: from (self . 0) } }
    };
}

impl_410!()