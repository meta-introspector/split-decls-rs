macro_rules! deps {
    () => {
        CodePointMapDataBorrowed!();
        GeneralCategoryGroup!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a > CodePointMapDataBorrowed < 'a , GeneralCategory > { # [doc = " Yields an [`Iterator`] returning ranges of consecutive code points that"] # [doc = " have a `General_Category` value belonging to the specified [`GeneralCategoryGroup`]"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};"] # [doc = " use icu::properties::CodePointMapData;"] # [doc = ""] # [doc = " let gc = CodePointMapData::<GeneralCategory>::new();"] # [doc = " let mut ranges = gc.iter_ranges_for_group(GeneralCategoryGroup::Letter);"] # [doc = " assert_eq!(ranges.next().unwrap(), 'A' as u32..='Z' as u32);"] # [doc = " assert_eq!(ranges.next().unwrap(), 'a' as u32..='z' as u32);"] # [doc = " assert_eq!(ranges.next().unwrap(), 'ª' as u32..='ª' as u32);"] # [doc = " assert_eq!(ranges.next().unwrap(), 'µ' as u32..='µ' as u32);"] # [doc = " assert_eq!(ranges.next().unwrap(), 'º' as u32..='º' as u32);"] # [doc = " assert_eq!(ranges.next().unwrap(), 'À' as u32..='Ö' as u32);"] # [doc = " assert_eq!(ranges.next().unwrap(), 'Ø' as u32..='ö' as u32);"] # [doc = " ```"] pub fn iter_ranges_for_group (self , group : GeneralCategoryGroup ,) -> impl Iterator < Item = RangeInclusive < u32 > > + 'a { self . map . iter_ranges_mapped (move | value | group . contains (value)) . filter (| v | v . value) . map (| v | v . range) } }
    };
}

impl_16!();