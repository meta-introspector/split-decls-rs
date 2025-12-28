macro_rules! deps {
    () => {
        HangulSyllableType!();
    };
}

macro_rules! macro_109 {
    () => {
        deps!();
        create_const_array ! { # [allow (non_upper_case_globals)] impl HangulSyllableType { # [doc = " (`NA`) not applicable (e.g. not a Hangul code point)."] pub const NotApplicable : HangulSyllableType = HangulSyllableType (0) ; # [doc = " (`L`) a conjoining leading consonant Jamo."] pub const LeadingJamo : HangulSyllableType = HangulSyllableType (1) ; # [doc = " (`V`) a conjoining vowel Jamo."] pub const VowelJamo : HangulSyllableType = HangulSyllableType (2) ; # [doc = " (`T`) a conjoining trailing consonant Jamo."] pub const TrailingJamo : HangulSyllableType = HangulSyllableType (3) ; # [doc = " (`LV`) a precomposed syllable with a leading consonant and a vowel."] pub const LeadingVowelSyllable : HangulSyllableType = HangulSyllableType (4) ; # [doc = " (`LVT`) a precomposed syllable with a leading consonant, a vowel, and a trailing consonant."] pub const LeadingVowelTrailingSyllable : HangulSyllableType = HangulSyllableType (5) ; } }
    };
}

macro_109!()