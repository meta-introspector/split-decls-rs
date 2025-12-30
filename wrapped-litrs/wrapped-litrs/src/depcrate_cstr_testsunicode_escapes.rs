// Generated macro for unicode_escapes (function)
macro_rules! Depcrate_cstr_testsunicode_escapes {
() => {
// Module: crate::cstr::tests
// Provides: {"unicode_escapes"}
// Dependencies: {}
# [test] fn unicode_escapes () { check ! (c"\u{b} " , true , None) ; check ! (c" \u{B} " , true , None) ; check ! (c"\u{7e}" , true , None) ; check ! (c"నక్క\u{E4}" , true , None) ; check ! (c"\u{e4} నక్క" , true , None) ; check ! (c" \u{fc}నక్క " , true , None) ; check ! (c"\u{Fc}" , true , None) ; check ! (c"\u{fC}🦊\nлиса" , true , None) ; check ! (c"лиса\u{FC}" , true , None) ; check ! (c"лиса\u{b10}నక్క🦊" , true , None) ; check ! (c"\"నక్క\u{B10}" , true , None) ; check ! (c"лиса\\\u{0b10}" , true , None) ; check ! (c"ли🦊са\\\"\u{0b10}" , true , None) ; check ! (c"నక్క\\\\u{0b10}" , true , None) ; check ! (c"\u{2764}Füchsin" , true , None) ; check ! (c"Füchse \u{1f602}" , true , None) ; check ! (c"cd\u{1F602}ab" , true , None) ; check ! (c"\\🦊\u{3_b}" , true , None) ; check ! (c"🦊\u{1_F_6_0_2}Füchsin" , true , None) ; check ! (c"నక్క\\\u{1_F6_02_____}నక్క" , true , None) ; }
};
}
