macro_rules! unicode_escapes {
    () => {
        # [test] fn unicode_escapes () { check ! ("\u{0}" , true , None) ; check ! (" \u{00}" , true , None) ; check ! ("\u{b} " , true , None) ; check ! (" \u{B} " , true , None) ; check ! ("\u{7e}" , true , None) ; check ! ("నక్క\u{E4}" , true , None) ; check ! ("\u{e4} నక్క" , true , None) ; check ! (" \u{fc}నక్క " , true , None) ; check ! ("\u{Fc}" , true , None) ; check ! ("\u{fC}🦊\nлиса" , true , None) ; check ! ("лиса\u{FC}" , true , None) ; check ! ("лиса\u{b10}నక్క🦊" , true , None) ; check ! ("\"నక్క\u{B10}" , true , None) ; check ! ("лиса\\\u{0b10}" , true , None) ; check ! ("ли🦊са\\\"\u{0b10}" , true , None) ; check ! ("నక్క\\\\u{0b10}" , true , None) ; check ! ("\u{2764}Füchsin" , true , None) ; check ! ("Füchse \u{1f602}" , true , None) ; check ! ("cd\u{1F602}ab" , true , None) ; check ! ("\u{0}🦊" , true , None) ; check ! ("лиса\u{0__}" , true , None) ; check ! ("\\🦊\u{3_b}" , true , None) ; check ! ("🦊\u{1_F_6_0_2}Füchsin" , true , None) ; check ! ("నక్క\\\u{1_F6_02_____}నక్క" , true , None) ; check ! ("a\u{7e}b\u{fc}c\u{0b10}d" , true , None) ; }
    };
}

unicode_escapes!();