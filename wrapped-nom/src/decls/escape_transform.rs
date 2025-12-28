macro_rules! deps {
    () => {
        EscapedTransform!();
        Parser!();
        ErrorKind!();
        Error!();
        IResult!();
        Tag!();
        Err!();
    };
}

macro_rules! escape_transform {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [test] fn escape_transform () { use crate :: Parser ; fn esc (i : & [u8]) -> IResult < & [u8] , String > { map (escaped_transform (alpha , '\\' , alt ((value (& b"\\" [..] , tag ("\\")) , value (& b"\"" [..] , tag ("\"")) , value (& b"\n" [..] , tag ("n")) ,)) ,) , to_s ,) . parse (i) } assert_eq ! (esc (& b"abcd;" [..]) , Ok ((& b";" [..] , String :: from ("abcd")))) ; assert_eq ! (esc (& b"ab\\\"cd;" [..]) , Ok ((& b";" [..] , String :: from ("ab\"cd")))) ; assert_eq ! (esc (& b"\\\"abcd;" [..]) , Ok ((& b";" [..] , String :: from ("\"abcd")))) ; assert_eq ! (esc (& b"\\n;" [..]) , Ok ((& b";" [..] , String :: from ("\n")))) ; assert_eq ! (esc (& b"ab\\\"12" [..]) , Ok ((& b"12" [..] , String :: from ("ab\"")))) ; assert_eq ! (esc (& b"AB\\" [..]) , Err (Err :: Error (error_position ! (& b"\\" [..] , ErrorKind :: EscapedTransform)))) ; assert_eq ! (esc (& b"AB\\A" [..]) , Err (Err :: Error (error_node_position ! (& b"AB\\A" [..] , ErrorKind :: EscapedTransform , error_position ! (& b"A" [..] , ErrorKind :: Tag))))) ; fn esc2 (i : & [u8]) -> IResult < & [u8] , String > { map (escaped_transform (alpha , '&' , alt ((value ("è" . as_bytes () , tag ("egrave;")) , value ("à" . as_bytes () , tag ("agrave;")) ,)) ,) , to_s ,) . parse (i) } assert_eq ! (esc2 (& b"ab&egrave;DEF;" [..]) , Ok ((& b";" [..] , String :: from ("abèDEF")))) ; assert_eq ! (esc2 (& b"ab&egrave;D&agrave;EF;" [..]) , Ok ((& b";" [..] , String :: from ("abèDàEF")))) ; }
    };
}

escape_transform!()