macro_rules! deps {
    () => {
        IResult!();
    };
}

macro_rules! take_while_m_n_utf8_full_match {
    () => {
        deps!();
        # [test] fn take_while_m_n_utf8_full_match () { use crate :: bytes :: streaming :: take_while_m_n ; fn parser (i : & str) -> IResult < & str , & str > { take_while_m_n (1 , 1 , | c : char | c . is_alphabetic ()) (i) } assert_eq ! (parser ("øn") , Ok (("n" , "ø"))) ; }
    };
}

take_while_m_n_utf8_full_match!();