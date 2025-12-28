macro_rules! deps {
    () => {
        Err!();
        Error!();
        Complete!();
        IResult!();
        ErrorKind!();
    };
}

macro_rules! length_value_test {
    () => {
        deps!();
        # [test] fn length_value_test () { fn length_value_1 (i : & [u8]) -> IResult < & [u8] , u16 > { length_value (be_u8 , be_u16) . parse (i) } fn length_value_2 (i : & [u8]) -> IResult < & [u8] , (u8 , u8) > { length_value (be_u8 , (be_u8 , be_u8)) . parse (i) } let i1 = [0 , 5 , 6] ; assert_eq ! (length_value_1 (& i1) , Err (Err :: Error (error_position ! (& b"" [..] , ErrorKind :: Complete)))) ; assert_eq ! (length_value_2 (& i1) , Err (Err :: Error (error_position ! (& b"" [..] , ErrorKind :: Complete)))) ; let i2 = [1 , 5 , 6 , 3] ; assert_eq ! (length_value_1 (& i2) , Err (Err :: Error (error_position ! (& i2 [1 .. 2] , ErrorKind :: Complete)))) ; assert_eq ! (length_value_2 (& i2) , Err (Err :: Error (error_position ! (& i2 [1 .. 2] , ErrorKind :: Complete)))) ; let i3 = [2 , 5 , 6 , 3 , 4 , 5 , 7] ; assert_eq ! (length_value_1 (& i3) , Ok ((& i3 [3 ..] , 1286))) ; assert_eq ! (length_value_2 (& i3) , Ok ((& i3 [3 ..] , (5 , 6)))) ; let i4 = [3 , 5 , 6 , 3 , 4 , 5] ; assert_eq ! (length_value_1 (& i4) , Ok ((& i4 [4 ..] , 1286))) ; assert_eq ! (length_value_2 (& i4) , Ok ((& i4 [4 ..] , (5 , 6)))) ; }
    };
}

length_value_test!();