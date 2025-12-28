macro_rules! deps {
    () => {
        IntegerLit!();
        IntegerBase!();
        FromIntegerLiteral!();
    };
}

macro_rules! non_standard_suffixes {
    () => {
        deps!();
        # [test] fn non_standard_suffixes () { # [track_caller] fn check_suffix < T : FromIntegerLiteral + PartialEq + Debug + Display > (input : & str , value : T , base : IntegerBase , main_part : & str , suffix : & str ,) { check (input , value , base , main_part , None) ; assert_eq ! (IntegerLit :: parse (input) . unwrap () . suffix () , suffix) ; } check_suffix ("5u7" , 5 , Decimal , "5" , "u7") ; check_suffix ("5u7" , 5 , Decimal , "5" , "u7") ; check_suffix ("5u9" , 5 , Decimal , "5" , "u9") ; check_suffix ("5u0" , 5 , Decimal , "5" , "u0") ; check_suffix ("33u12" , 33 , Decimal , "33" , "u12") ; check_suffix ("84u17" , 84 , Decimal , "84" , "u17") ; check_suffix ("99u80" , 99 , Decimal , "99" , "u80") ; check_suffix ("1234uu16" , 1234 , Decimal , "1234" , "uu16") ; check_suffix ("5i7" , 5 , Decimal , "5" , "i7") ; check_suffix ("5i9" , 5 , Decimal , "5" , "i9") ; check_suffix ("5i0" , 5 , Decimal , "5" , "i0") ; check_suffix ("33i12" , 33 , Decimal , "33" , "i12") ; check_suffix ("84i17" , 84 , Decimal , "84" , "i17") ; check_suffix ("99i80" , 99 , Decimal , "99" , "i80") ; check_suffix ("1234ii16" , 1234 , Decimal , "1234" , "ii16") ; check_suffix ("0ui32" , 0 , Decimal , "0" , "ui32") ; check_suffix ("1iu32" , 1 , Decimal , "1" , "iu32") ; check_suffix ("54321a64" , 54321 , Decimal , "54321" , "a64") ; check_suffix ("54321b64" , 54321 , Decimal , "54321" , "b64") ; check_suffix ("54321x64" , 54321 , Decimal , "54321" , "x64") ; check_suffix ("54321o64" , 54321 , Decimal , "54321" , "o64") ; check_suffix ("0a" , 0 , Decimal , "0" , "a") ; check_suffix ("0a3" , 0 , Decimal , "0" , "a3") ; check_suffix ("0z" , 0 , Decimal , "0" , "z") ; check_suffix ("0z3" , 0 , Decimal , "0" , "z3") ; check_suffix ("0b0a" , 0 , Binary , "0" , "a") ; check_suffix ("0b0A" , 0 , Binary , "0" , "A") ; check_suffix ("0b01f" , 1 , Binary , "01" , "f") ; check_suffix ("0b01F" , 1 , Binary , "01" , "F") ; check_suffix ("0o7a_" , 7 , Octal , "7" , "a_") ; check_suffix ("0o7A_" , 7 , Octal , "7" , "A_") ; check_suffix ("0o72f_0" , 0o72 , Octal , "72" , "f_0") ; check_suffix ("0o72F_0" , 0o72 , Octal , "72" , "F_0") ; check_suffix ("0x8cg" , 0x8c , Hexadecimal , "8c" , "g") ; check_suffix ("0x8cG" , 0x8c , Hexadecimal , "8c" , "G") ; check_suffix ("0x8c1h_" , 0x8c1 , Hexadecimal , "8c1" , "h_") ; check_suffix ("0x8c1H_" , 0x8c1 , Hexadecimal , "8c1" , "H_") ; check_suffix ("0x8czu16" , 0x8c , Hexadecimal , "8c" , "zu16") ; check_suffix ("123_foo" , 123 , Decimal , "123_" , "foo") ; }
    };
}

non_standard_suffixes!()