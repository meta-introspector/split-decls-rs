macro_rules! deps {
    () => {
        IndicSyllabicCategory!();
    };
}

macro_rules! macro_142 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Indic_Syllabic_Category" ; short_name : "InSC" ; ident : IndicSyllabicCategory ; data_marker : crate :: provider :: PropertyEnumIndicSyllabicCategoryV1 ; singleton : SINGLETON_PROPERTY_ENUM_INDIC_SYLLABIC_CATEGORY_V1 ; ule_ty : u8 ; }
    };
}

macro_142!();