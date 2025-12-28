macro_rules! deps {
    () => {
        ErrorKind!();
        IResult!();
        Needed!();
        Tag!();
        Error!();
        Err!();
    };
}

macro_rules! separated_pair_test {
    () => {
        deps!();
        # [test] fn separated_pair_test () { fn sep_pair_abc_def (i : & [u8]) -> IResult < & [u8] , (& [u8] , & [u8]) > { separated_pair (tag ("abc") , tag (",") , tag ("def")) . parse (i) } assert_eq ! (sep_pair_abc_def (& b"abc,defghijkl" [..]) , Ok ((& b"ghijkl" [..] , (& b"abc" [..] , & b"def" [..])))) ; assert_eq ! (sep_pair_abc_def (& b"ab" [..]) , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (sep_pair_abc_def (& b"abc,d" [..]) , Err (Err :: Incomplete (Needed :: new (2)))) ; assert_eq ! (sep_pair_abc_def (& b"xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; assert_eq ! (sep_pair_abc_def (& b"xxx,def" [..]) , Err (Err :: Error (error_position ! (& b"xxx,def" [..] , ErrorKind :: Tag)))) ; assert_eq ! (sep_pair_abc_def (& b"abc,xxx" [..]) , Err (Err :: Error (error_position ! (& b"xxx" [..] , ErrorKind :: Tag)))) ; }
    };
}

separated_pair_test!();