macro_rules! non_ascii_escapes {
    () => {
        # [test] fn non_ascii_escapes () { check ! (b"\x80" , true , None) ; check ! (b"\x8a" , true , None) ; check ! (b"\x8C" , true , None) ; check ! (b"\x99" , true , None) ; check ! (b"\xa0" , true , None) ; check ! (b"\xAd" , true , None) ; check ! (b"\xfe" , true , None) ; check ! (b"\xFe" , true , None) ; check ! (b"\xfF" , true , None) ; check ! (b"\xFF" , true , None) ; }
    };
}

non_ascii_escapes!()