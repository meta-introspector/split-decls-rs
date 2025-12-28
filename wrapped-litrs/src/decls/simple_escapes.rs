macro_rules! simple_escapes {
    () => {
        # [test] fn simple_escapes () { check ! ("a\nb" , true , None) ; check ! ("\nb" , true , None) ; check ! ("a\n" , true , None) ; check ! ("\n" , true , None) ; check ! ("\x60犬 \t 猫\r馬\n うさぎ \0ネズミ" , true , None) ; check ! ("నా \\పిల్లి లావుగా ఉంది" , true , None) ; check ! ("నా \\పిల్లి లావుగా 🐈\"ఉంది" , true , None) ; check ! ("\\నా\\ పిల్లి లావుగా\" ఉంది\"" , true , None) ; check ! ("\"నా \\🐈 పిల్లి లావుగా \" ఉంది\\" , true , None) ; check ! ("\x00" , true , None) ; check ! (" \x01" , true , None) ; check ! ("\x0c 🦊" , true , None) ; check ! (" 🦊\x0D " , true , None) ; check ! ("\\x13" , true , None) ; check ! ("\"x30" , true , None) ; }
    };
}

simple_escapes!()