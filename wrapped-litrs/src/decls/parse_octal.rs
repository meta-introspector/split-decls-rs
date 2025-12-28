macro_rules! parse_octal {
    () => {
        # [test] fn parse_octal () { check ("0o0" , 0o0 , Octal , "0" , None) ; check ("0o1" , 0o1 , Octal , "1" , None) ; check ("0o6" , 0o6 , Octal , "6" , None) ; check ("0o7" , 0o7 , Octal , "7" , None) ; check ("0o17" , 0o17 , Octal , "17" , None) ; check ("0o123" , 0o123 , Octal , "123" , None) ; check ("0o7654321" , 0o7654321 , Octal , "7654321" , None) ; check ("0o7_53_1" , 0o7_53_1 , Octal , "7_53_1" , None) ; check ("0o66_" , 0o66_ , Octal , "66_" , None) ; check ("0o755u16" , 0o755u16 , Octal , "755" , Some (Ty :: U16)) ; check ("0o755i128" , 0o755i128 , Octal , "755" , Some (Ty :: I128)) ; }
    };
}

parse_octal!();