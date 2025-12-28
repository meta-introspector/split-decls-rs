macro_rules! starting_underscore {
    () => {
        # [test] fn starting_underscore () { check ("0b_1" , 1 , Binary , "_1" , None) ; check ("0b_010i16" , 0b_010 , Binary , "_010" , Some (Ty :: I16)) ; check ("0o_5" , 5 , Octal , "_5" , None) ; check ("0o_750u128" , 0o_750u128 , Octal , "_750" , Some (Ty :: U128)) ; check ("0x_c" , 0xc , Hexadecimal , "_c" , None) ; check ("0x_cf3i8" , 0x_cf3 , Hexadecimal , "_cf3" , Some (Ty :: I8)) ; }
    };
}

starting_underscore!();