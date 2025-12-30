// Generated macro for PluralRulesWithRanges (struct)
macro_rules! DepcratePluralRulesWithRanges {
() => {
// Module: crate
// Provides: {"PluralRulesWithRanges"}
// Dependencies: {}
# [doc = " A [`PluralRules`] that also has the ability to retrieve an appropriate [`Plural Category`] for a"] # [doc = " range."] # [doc = ""] # [doc = " ✨ *Enabled with the `experimental` Cargo feature.*"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Use with caution."] # [doc = " <a href=\"https://github.com/unicode-org/icu4x/issues/4140\">#4140</a>"] # [doc = " </div>"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::locale;"] # [doc = " use icu::plurals::{PluralCategory, PluralOperands, PluralRulesWithRanges};"] # [doc = ""] # [doc = " let ranges = PluralRulesWithRanges::try_new("] # [doc = "     locale!(\"ar\").into(),"] # [doc = "     Default::default(),"] # [doc = " )"] # [doc = " .expect(\"locale should be present\");"] # [doc = ""] # [doc = " let operands = PluralOperands::from(1_usize);"] # [doc = " let operands2: PluralOperands ="] # [doc = "     \"2.0\".parse().expect(\"parsing to operands should succeed\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     ranges.category_for_range(operands, operands2),"] # [doc = "     PluralCategory::Other"] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " [`Plural Category`]: PluralCategory"] # [cfg (feature = "experimental")] # [derive (Debug)] pub struct PluralRulesWithRanges < R > { rules : R , ranges : DataPayload < PluralsRangesV1 > , }
};
}
