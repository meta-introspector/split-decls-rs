macro_rules! deps {
    () => {
        CodePointMapDataBorrowed!();
        CodePointSetData!();
        GeneralCategoryGroup!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl CodePointMapDataBorrowed < '_ , GeneralCategory > { # [doc = " Get a [`CodePointSetData`] for all elements corresponding to a particular value group"] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};"] # [doc = " use icu::properties::CodePointMapData;"] # [doc = ""] # [doc = " let gc = CodePointMapData::<GeneralCategory>::new();"] # [doc = ""] # [doc = " let other_letter_set_data ="] # [doc = "     gc.get_set_for_value_group(GeneralCategoryGroup::OtherLetter);"] # [doc = " let other_letter_set = other_letter_set_data.as_borrowed();"] # [doc = ""] # [doc = " assert!(other_letter_set.contains('木')); // U+6728"] # [doc = " assert!(!other_letter_set.contains('🎃')); // U+1F383 JACK-O-LANTERN"] # [doc = " ```"] # [cfg (feature = "alloc")] pub fn get_set_for_value_group (self , value : GeneralCategoryGroup) -> crate :: CodePointSetData { let matching_gc_ranges = self . iter_ranges () . filter (| cpm_range | (1 << cpm_range . value as u32) & value . 0 != 0) . map (| cpm_range | cpm_range . range) ; CodePointSetData :: from_code_point_inversion_list (matching_gc_ranges . collect ()) } }
    };
}

impl_13!()