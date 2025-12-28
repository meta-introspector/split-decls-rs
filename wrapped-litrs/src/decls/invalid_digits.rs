macro_rules! deps {
    () => {
        IntegerLit!();
    };
}

macro_rules! invalid_digits {
    () => {
        deps!();
        # [test] fn invalid_digits () { assert_err ! (IntegerLit , "0b10201" , InvalidDigit , 4) ; assert_err ! (IntegerLit , "0b9" , InvalidDigit , 2) ; assert_err ! (IntegerLit , "0b07" , InvalidDigit , 3) ; assert_err ! (IntegerLit , "0o12380" , InvalidDigit , 5) ; assert_err ! (IntegerLit , "0o192" , InvalidDigit , 3) ; assert_err_single ! (IntegerLit :: parse ("a_123") , DoesNotStartWithDigit , 0) ; assert_err_single ! (IntegerLit :: parse ("B_123") , DoesNotStartWithDigit , 0) ; }
    };
}

invalid_digits!()