macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! common_edges {
    () => {
        deps!();
        pub fn common_edges (file1 : & [Token] , file2 : & [Token]) -> (u32 , u32) { let prefix = common_prefix (file1 , file2) ; let postfix = common_postfix (& file1 [prefix as usize ..] , & file2 [prefix as usize ..]) ; (prefix , postfix) }
    };
}

common_edges!();