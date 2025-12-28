macro_rules! suffixes {
    () => {
        # [test] fn suffixes () { check ! ("hello" , r###""hello"suffix"### , false , None , "suffix") ; check ! (r"お前はもう死んでいる" , r###"r"お前はもう死んでいる"_banana"### , false , Some (0) , "_banana") ; check ! ("fox" , r#""fox"peter"# , false , None , "peter") ; check ! ("🦊" , r#""🦊"peter"# , false , None , "peter") ; check ! ("నక్క\\\\u{0b10}" , r###""నక్క\\\\u{0b10}"jü_rgen"### , true , None , "jü_rgen") ; }
    };
}

suffixes!();