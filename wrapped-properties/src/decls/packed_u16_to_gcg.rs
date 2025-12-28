macro_rules! deps {
    () => {
        GeneralCategoryGroup!();
    };
}

macro_rules! packed_u16_to_gcg {
    () => {
        deps!();
        fn packed_u16_to_gcg (value : u16) -> GeneralCategoryGroup { match value { 0xFFFF => GeneralCategoryGroup :: CasedLetter , 0xFFFE => GeneralCategoryGroup :: Letter , 0xFFFD => GeneralCategoryGroup :: Mark , 0xFFFC => GeneralCategoryGroup :: Number , 0xFFFB => GeneralCategoryGroup :: Separator , 0xFFFA => GeneralCategoryGroup :: Other , 0xFFF9 => GeneralCategoryGroup :: Punctuation , 0xFFF8 => GeneralCategoryGroup :: Symbol , v if v < 32 => GeneralCategory :: new_from_u8 (v as u8) . map (| gc | gc . into ()) . unwrap_or (GeneralCategoryGroup (0)) , _ => GeneralCategoryGroup (0) , } }
    };
}

packed_u16_to_gcg!()