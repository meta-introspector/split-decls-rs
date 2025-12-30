// Generated macro for tests (module)
macro_rules! Depcrate_personnames_specifications_derive_name_ordertests {
() => {
// Module: crate::personnames::specifications::derive_name_order
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use icu_locale :: LocaleFallbacker ; use icu_locale_core :: locale ; use zerovec :: VarZeroVec ; use super :: name_order_derive ; use crate :: personnames :: api :: FormattingOrder ; # [test] fn test_plain_locale () { let fallbacker = LocaleFallbacker :: new () ; let given_first = VarZeroVec :: from (& ["und"]) ; let surname_first = VarZeroVec :: from (& ["hu" , "ja" , "km" , "ko" , "mn" , "vi" , "yue" , "zh"]) ; assert_eq ! (name_order_derive (& locale ! ("de-Latn-ch") , & surname_first , & given_first , fallbacker) , FormattingOrder :: GivenFirst , "failed for de_Latn_ch") ; assert_eq ! (name_order_derive (& locale ! ("ja-Jpan-jp") , & surname_first , & given_first , fallbacker) , FormattingOrder :: GivenFirst , "failed for ja_Jpan_jp") ; } # [test] fn test_mixed_und () { let fallbacker = LocaleFallbacker :: new () ; let given_first = VarZeroVec :: from (& ["und"]) ; let surname_first = VarZeroVec :: from (& ["zh" , "ja" , "und-CN" , "und-TW" , "und-SG" , "und-HK" , "und-MO" , "und-HU" , "und-JP" ,]) ; assert_eq ! (name_order_derive (& locale ! ("en-Latn-SG") , & surname_first , & given_first , fallbacker) , FormattingOrder :: SurnameFirst , "failed for en_Latn_SG") ; assert_eq ! (name_order_derive (& locale ! ("zh-Hans-CN") , & surname_first , & given_first , fallbacker) , FormattingOrder :: SurnameFirst , "failed for zh_Hans_CN") ; assert_eq ! (name_order_derive (& locale ! ("zh-Hans") , & surname_first , & given_first , fallbacker) , FormattingOrder :: GivenFirst , "failed for zh_Hans") ; } }
};
}
