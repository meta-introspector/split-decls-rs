macro_rules! deps {
    () => {
        AsChar!();
        IResult!();
    };
}

macro_rules! recognize_take_while {
    () => {
        deps!();
        # [test] # [cfg (feature = "std")] fn recognize_take_while () { use crate :: bytes :: streaming :: take_while ; use crate :: combinator :: recognize ; fn x (i : & [u8]) -> IResult < & [u8] , & [u8] > { take_while (AsChar :: is_alphanum) (i) } fn y (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (x) . parse (i) } assert_eq ! (x (& b"ab." [..]) , Ok ((& b"." [..] , & b"ab" [..]))) ; println ! ("X: {:?}" , x (& b"ab" [..])) ; assert_eq ! (y (& b"ab." [..]) , Ok ((& b"." [..] , & b"ab" [..]))) ; }
    };
}

recognize_take_while!()