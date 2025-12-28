macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! round_up_to_alignment {
    () => {
        deps!();
        fn round_up_to_alignment < 'll > (bx : & mut Builder < '_ , 'll , '_ > , mut value : & 'll Value , align : Align ,) -> & 'll Value { value = bx . add (value , bx . cx () . const_i32 (align . bytes () as i32 - 1)) ; return bx . and (value , bx . cx () . const_i32 (- (align . bytes () as i32))) ; }
    };
}

round_up_to_alignment!();