macro_rules! deps {
    () => {
        CodePointMapData!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl GeneralCategoryFunc for & '_ CodePointMapData < GeneralCategory > { fn general_category (& self , ch : char) -> harfbuzz_traits :: GeneralCategory { GeneralCategoryFunc :: general_category (& self . as_borrowed () , ch) } }
    };
}

impl_426!();