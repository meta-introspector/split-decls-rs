macro_rules! deps {
    () => {
        ErrorCode!();
    };
}

macro_rules! check_recursion {
    () => {
        deps!();
        macro_rules ! check_recursion { ($ this : ident $ ($ body : tt) *) => { if_checking_recursion_limit ! { $ this . remaining_depth -= 1 ; if $ this . remaining_depth == 0 { return Err ($ this . peek_error (ErrorCode :: RecursionLimitExceeded)) ; } } $ this $ ($ body) * if_checking_recursion_limit ! { $ this . remaining_depth += 1 ; } } ; }
    };
}

check_recursion!()