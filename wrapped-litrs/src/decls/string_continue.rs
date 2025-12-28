macro_rules! deps {
    () => {
        StringLit!();
    };
}

macro_rules! string_continue {
    () => {
        deps!();
        # [test] fn string_continue () { check ! ("నక్క\
        bar" , true , None) ; check ! ("foo\
🦊" , true , None) ; check ! ("foo\

        banana" , true , None) ; let lit = StringLit :: parse ("\"foo\\\n\t\n \n\tbar\"") . expect ("failed to parse") ; assert_eq ! (lit . value () , "foobar") ; let lit = StringLit :: parse ("\"foo\\\n\u{85}bar\"") . expect ("failed to parse") ; assert_eq ! (lit . value () , "foo\u{85}bar") ; let lit = StringLit :: parse ("\"foo\\\n\u{a0}bar\"") . expect ("failed to parse") ; assert_eq ! (lit . value () , "foo\u{a0}bar") ; check ! (r"foo\
        bar" , false , Some (0)) ; }
    };
}

string_continue!()