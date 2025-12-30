// Generated macro for no_valid_digits (function)
macro_rules! Depcrate_integer_testsno_valid_digits {
() => {
// Module: crate::integer::tests
// Provides: {"no_valid_digits"}
// Dependencies: {}
# [test] fn no_valid_digits () { assert_err ! (IntegerLit , "0x_" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0x__" , NoDigits , 2 .. 4) ; assert_err ! (IntegerLit , "0x________" , NoDigits , 2 .. 10) ; assert_err ! (IntegerLit , "0x_i8" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0x_u8" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0x_isize" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0x_usize" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0o_" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0o__" , NoDigits , 2 .. 4) ; assert_err ! (IntegerLit , "0o________" , NoDigits , 2 .. 10) ; assert_err ! (IntegerLit , "0o_i32" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0o_u32" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0b_" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0b__" , NoDigits , 2 .. 4) ; assert_err ! (IntegerLit , "0b________" , NoDigits , 2 .. 10) ; assert_err ! (IntegerLit , "0b_i128" , NoDigits , 2 .. 3) ; assert_err ! (IntegerLit , "0b_u128" , NoDigits , 2 .. 3) ; }
};
}
