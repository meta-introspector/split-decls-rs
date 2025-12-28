macro_rules! GeneralCategoryGroup {
    () => {
        # [doc = " Groupings of multiple General_Category property values."] # [doc = ""] # [doc = " Instances of `GeneralCategoryGroup` represent the defined multi-category"] # [doc = " values that are useful for users in certain contexts, such as regex. In"] # [doc = " other words, unlike [`GeneralCategory`], this supports groups of general"] # [doc = " categories: for example, `Letter` /// is the union of `UppercaseLetter`,"] # [doc = " `LowercaseLetter`, etc."] # [doc = ""] # [doc = " See <https://www.unicode.org/reports/tr44/> ."] # [doc = ""] # [doc = " The discriminants correspond to the `U_GC_XX_MASK` constants in ICU4C."] # [doc = " Unlike [`GeneralCategory`], this supports groups of general categories: for example, `Letter`"] # [doc = " is the union of `UppercaseLetter`, `LowercaseLetter`, etc."] # [doc = ""] # [doc = " See `UCharCategory` and `U_GET_GC_MASK` in ICU4C."] # [derive (Copy , Clone , PartialEq , Debug , Eq)] # [allow (clippy :: exhaustive_structs)] # [repr (transparent)] pub struct GeneralCategoryGroup (pub (crate) u32) ;
    };
}

GeneralCategoryGroup!()