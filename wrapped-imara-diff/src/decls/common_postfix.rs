macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! common_postfix {
    () => {
        deps!();
        pub fn common_postfix (file1 : & [Token] , file2 : & [Token]) -> u32 { let mut off = 0 ; for (token1 , token2) in file1 . iter () . rev () . zip (file2 . iter () . rev ()) { if token1 != token2 { break ; } off += 1 ; } off }
    };
}

common_postfix!();