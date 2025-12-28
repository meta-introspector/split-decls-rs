macro_rules! deps {
    () => {
        CodePointMapDataBorrowed!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl GeneralCategoryFunc for CodePointMapDataBorrowed < '_ , GeneralCategory > { fn general_category (& self , ch : char) -> harfbuzz_traits :: GeneralCategory { self . get (ch) . into () } }
    };
}

impl_424!();