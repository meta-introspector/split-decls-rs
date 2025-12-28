macro_rules! deps {
    () => {
        Error!();
        EscapedTransform!();
        Err!();
        Tag!();
        ErrorKind!();
        IResult!();
    };
}

macro_rules! escape_transform_str {
    () => {
        deps!();
        # [cfg (feature = "std")] # [test] fn escape_transform_str () { fn esc (i : & str) -> IResult < & str , String > { escaped_transform (alpha , '\\' , alt ((value ("\\" , tag ("\\")) , value ("\"" , tag ("\"")) , value ("\n" , tag ("n")) ,)) ,) (i) } assert_eq ! (esc ("abcd;") , Ok ((";" , String :: from ("abcd")))) ; assert_eq ! (esc ("ab\\\"cd;") , Ok ((";" , String :: from ("ab\"cd")))) ; assert_eq ! (esc ("\\\"abcd;") , Ok ((";" , String :: from ("\"abcd")))) ; assert_eq ! (esc ("\\n;") , Ok ((";" , String :: from ("\n")))) ; assert_eq ! (esc ("ab\\\"12") , Ok (("12" , String :: from ("ab\"")))) ; assert_eq ! (esc ("AB\\") , Err (Err :: Error (error_position ! ("\\" , ErrorKind :: EscapedTransform)))) ; assert_eq ! (esc ("AB\\A") , Err (Err :: Error (error_node_position ! ("AB\\A" , ErrorKind :: EscapedTransform , error_position ! ("A" , ErrorKind :: Tag))))) ; fn esc2 (i : & str) -> IResult < & str , String > { escaped_transform (alpha , '&' , alt ((value ("è" , tag ("egrave;")) , value ("à" , tag ("agrave;")))) ,) (i) } assert_eq ! (esc2 ("ab&egrave;DEF;") , Ok ((";" , String :: from ("abèDEF")))) ; assert_eq ! (esc2 ("ab&egrave;D&agrave;EF;") , Ok ((";" , String :: from ("abèDàEF")))) ; fn esc3 (i : & str) -> IResult < & str , String > { escaped_transform (alpha , '␛' , alt ((value ("\0" , tag ("0")) , value ("\n" , tag ("n")))) ,) (i) } assert_eq ! (esc3 ("a␛0bc␛n") , Ok (("" , String :: from ("a\0bc\n")))) ; }
    };
}

escape_transform_str!()