macro_rules! deps {
    () => {
        ByteStringLit!();
        FloatLit!();
        ByteLit!();
        Literal!();
        IntegerLit!();
        BoolLit!();
        CharLit!();
        StringLit!();
    };
}

macro_rules! assert_no_panic {
    () => {
        deps!();
        macro_rules ! assert_no_panic { ($ input : expr) => { let arr = $ input ; let input = std :: str :: from_utf8 (& arr) . expect ("not unicode") ; let res = std :: panic :: catch_unwind (move || { let _ = Literal :: parse (input) ; let _ = crate :: BoolLit :: parse (input) ; let _ = crate :: IntegerLit :: parse (input) ; let _ = crate :: FloatLit :: parse (input) ; let _ = crate :: CharLit :: parse (input) ; let _ = crate :: StringLit :: parse (input) ; let _ = crate :: ByteLit :: parse (input) ; let _ = crate :: ByteStringLit :: parse (input) ; }) ; if let Err (e) = res { println ! ("\n!!! panic for: {:?}" , input) ; std :: panic :: resume_unwind (e) ; } } ; }
    };
}

assert_no_panic!();