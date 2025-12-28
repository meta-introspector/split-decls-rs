macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! strip_common_prefix {
    () => {
        deps!();
        pub fn strip_common_prefix (file1 : & mut & [Token] , file2 : & mut & [Token]) -> u32 { let off = common_prefix (file1 , file2) ; * file1 = & file1 [off as usize ..] ; * file2 = & file2 [off as usize ..] ; off }
    };
}

strip_common_prefix!()