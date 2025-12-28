macro_rules! deps {
    () => {
        ImageSectionHeader!();
        SectionKind!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl pe :: ImageSectionHeader { pub (crate) fn kind (& self) -> SectionKind { let characteristics = self . characteristics . get (LE) ; if characteristics & (pe :: IMAGE_SCN_CNT_CODE | pe :: IMAGE_SCN_MEM_EXECUTE) != 0 { SectionKind :: Text } else if characteristics & pe :: IMAGE_SCN_CNT_INITIALIZED_DATA != 0 { if characteristics & pe :: IMAGE_SCN_MEM_DISCARDABLE != 0 { SectionKind :: Other } else if characteristics & pe :: IMAGE_SCN_MEM_WRITE != 0 { SectionKind :: Data } else { SectionKind :: ReadOnlyData } } else if characteristics & pe :: IMAGE_SCN_CNT_UNINITIALIZED_DATA != 0 { SectionKind :: UninitializedData } else if characteristics & pe :: IMAGE_SCN_LNK_INFO != 0 { SectionKind :: Linker } else { SectionKind :: Unknown } } }
    };
}

impl_223!()