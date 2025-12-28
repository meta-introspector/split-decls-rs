macro_rules! deps {
    () => {
        IResult!();
    };
}

macro_rules! take_while_m_n_utf8 {
    () => {
        deps!();
        # [test] fn take_while_m_n_utf8 () { use crate :: bytes :: streaming :: take_while_m_n ; fn parser (i : & str) -> IResult < & str , & str > { take_while_m_n (1 , 1 , | c | c == 'A' || c == '😃') (i) } assert_eq ! (parser ("A!") , Ok (("!" , "A"))) ; assert_eq ! (parser ("😃!") , Ok (("!" , "😃"))) ; }
    };
}

take_while_m_n_utf8!()