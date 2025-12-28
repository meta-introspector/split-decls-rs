macro_rules! deps {
    () => {
        IResult!();
    };
}

macro_rules! recognize {
    () => {
        deps!();
        # [test] fn recognize () { use crate :: bytes :: streaming :: { tag , take } ; use crate :: combinator :: recognize ; use crate :: sequence :: delimited ; fn x (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (delimited (tag ("<!--") , take (5_usize) , tag ("-->"))) . parse (i) } let r = x (& b"<!-- abc --> aaa" [..]) ; assert_eq ! (r , Ok ((& b" aaa" [..] , & b"<!-- abc -->" [..]))) ; let semicolon = & b";" [..] ; fn ya (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (alpha) . parse (i) } let ra = ya (& b"abc;" [..]) ; assert_eq ! (ra , Ok ((semicolon , & b"abc" [..]))) ; fn yd (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (digit) . parse (i) } let rd = yd (& b"123;" [..]) ; assert_eq ! (rd , Ok ((semicolon , & b"123" [..]))) ; fn yhd (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (hex_digit) . parse (i) } let rhd = yhd (& b"123abcDEF;" [..]) ; assert_eq ! (rhd , Ok ((semicolon , & b"123abcDEF" [..]))) ; fn yod (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (oct_digit) . parse (i) } let rod = yod (& b"1234567;" [..]) ; assert_eq ! (rod , Ok ((semicolon , & b"1234567" [..]))) ; fn ybd (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (bin_digit) . parse (i) } let rbd = ybd (& b"101010;" [..]) ; assert_eq ! (rbd , Ok ((semicolon , & b"101010" [..]))) ; fn yan (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (alphanumeric) . parse (i) } let ran = yan (& b"123abc;" [..]) ; assert_eq ! (ran , Ok ((semicolon , & b"123abc" [..]))) ; fn ys (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (space) . parse (i) } let rs = ys (& b" \t;" [..]) ; assert_eq ! (rs , Ok ((semicolon , & b" \t" [..]))) ; fn yms (i : & [u8]) -> IResult < & [u8] , & [u8] > { recognize (multispace) . parse (i) } let rms = yms (& b" \t\r\n;" [..]) ; assert_eq ! (rms , Ok ((semicolon , & b" \t\r\n" [..]))) ; }
    };
}

recognize!()