macro_rules! deps {
    () => {
        GeneralCategoryGroup!();
    };
}

macro_rules! gcg_to_packed_u16 {
    () => {
        deps!();
        fn gcg_to_packed_u16 (gcg : GeneralCategoryGroup) -> u16 { if gcg . 0 . is_power_of_two () { gcg . 0 . trailing_zeros () as u16 } else { match gcg { GeneralCategoryGroup :: CasedLetter => 0xFFFF , GeneralCategoryGroup :: Letter => 0xFFFE , GeneralCategoryGroup :: Mark => 0xFFFD , GeneralCategoryGroup :: Number => 0xFFFC , GeneralCategoryGroup :: Separator => 0xFFFB , GeneralCategoryGroup :: Other => 0xFFFA , GeneralCategoryGroup :: Punctuation => 0xFFF9 , GeneralCategoryGroup :: Symbol => 0xFFF8 , _ => 0xFF00 , } } }
    };
}

gcg_to_packed_u16!()