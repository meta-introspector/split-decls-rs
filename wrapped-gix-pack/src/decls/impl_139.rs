macro_rules! deps {
    () => {
        EntryDataMode!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl EntryDataMode { # [doc = " Returns true if a crc32 should be computed"] pub fn crc32 (& self) -> bool { match self { EntryDataMode :: KeepAndCrc32 | EntryDataMode :: Crc32 => true , EntryDataMode :: Keep | EntryDataMode :: Ignore => false , } } # [doc = " Returns true if compressed bytes should be kept"] pub fn keep (& self) -> bool { match self { EntryDataMode :: Keep | EntryDataMode :: KeepAndCrc32 => true , EntryDataMode :: Ignore | EntryDataMode :: Crc32 => false , } } }
    };
}

impl_139!()