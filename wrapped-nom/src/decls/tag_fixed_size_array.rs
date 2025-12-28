macro_rules! deps {
    () => {
        IResult!();
    };
}

macro_rules! tag_fixed_size_array {
    () => {
        deps!();
        # [test] fn tag_fixed_size_array () { use crate :: bytes :: streaming :: tag ; fn test (i : & [u8]) -> IResult < & [u8] , & [u8] > { tag (& [0x42] [..]) (i) } fn test2 (i : & [u8]) -> IResult < & [u8] , & [u8] > { tag (& [0x42] [..]) (i) } let input = [0x42 , 0x00] ; assert_eq ! (test (& input) , Ok ((& b"\x00" [..] , & b"\x42" [..]))) ; assert_eq ! (test2 (& input) , Ok ((& b"\x00" [..] , & b"\x42" [..]))) ; }
    };
}

tag_fixed_size_array!();