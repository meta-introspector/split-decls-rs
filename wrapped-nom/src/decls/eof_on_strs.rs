macro_rules! deps {
    () => {
        Error!();
        Err!();
        ErrorKind!();
    };
}

macro_rules! eof_on_strs {
    () => {
        deps!();
        # [test] fn eof_on_strs () { let not_over : & str = "Hello, world!" ; let is_over : & str = "" ; let res_not_over = eof (not_over) ; assert_parse ! (res_not_over , Err (Err :: Error (error_position ! (not_over , ErrorKind :: Eof)))) ; let res_over = eof (is_over) ; assert_parse ! (res_over , Ok ((is_over , is_over))) ; }
    };
}

eof_on_strs!();