macro_rules! byte_escapes {
    () => {
        # [test] fn byte_escapes () { check ! (b'\x80') ; check ! (b'\x8a') ; check ! (b'\x8C') ; check ! (b'\x99') ; check ! (b'\xa0') ; check ! (b'\xAd') ; check ! (b'\xfe') ; check ! (b'\xFe') ; check ! (b'\xfF') ; check ! (b'\xFF') ; }
    };
}

byte_escapes!();