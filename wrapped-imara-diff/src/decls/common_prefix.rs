macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! common_prefix {
    () => {
        deps!();
        pub fn common_prefix (file1 : & [Token] , file2 : & [Token]) -> u32 { let mut off = 0 ; for (token1 , token2) in file1 . iter () . zip (file2) { if token1 != token2 { break ; } off += 1 ; } off }
    };
}

common_prefix!()