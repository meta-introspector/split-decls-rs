macro_rules! deps {
    () => {
        ParamValue!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use crate :: ParamValue ; use crate :: { C_ATTR , C_ESCAPABLE , C_OWS , C_QDTEXT , C_TCHAR } ; # [doc = " Prints the character classes of all ASCII bytes from the table."] # [doc = ""] # [doc = " ```console"] # [doc = " $ cargo test -- --nocapture tests::table"] # [doc = " ```"] # [test] fn table () { println ! ("oct  dec  hex   char      tchar  qdtext  escapable  ows  attr") ; for b in 0 .. 128 { let classes = crate :: char_classes (b) ; let if_class = | class : u8 , label : & 'static str | if (classes & class) != 0 { label } else { "" } ; println ! ("{:03o}  {:>3}  0x{:02x}  {:8}  {:5}  {:6}  {:9}  {:3}  {:4}" , b , b , b , format ! ("{:?}" , char :: from (b)) , if_class (C_TCHAR , "tchar") , if_class (C_QDTEXT , "qdtext") , if_class (C_ESCAPABLE , "escapable") , if_class (C_OWS , "ows") , if_class (C_ATTR , "attr")) ; assert ! (classes & (C_TCHAR | C_QDTEXT) != C_TCHAR) ; assert ! (classes & (C_OWS | C_QDTEXT) != C_OWS) ; assert ! (classes & (C_QDTEXT | C_ESCAPABLE) != C_QDTEXT) ; } } # [test] fn try_from_escaped () { assert_eq ! (ParamValue :: try_from_escaped ("") . unwrap () . escapes , 0) ; assert_eq ! (ParamValue :: try_from_escaped ("foo") . unwrap () . escapes , 0) ; assert_eq ! (ParamValue :: try_from_escaped ("\\\"") . unwrap () . escapes , 1) ; assert_eq ! (ParamValue :: try_from_escaped ("foo\\\"bar") . unwrap () . escapes , 1) ; assert_eq ! (ParamValue :: try_from_escaped ("foo\\\"bar\\\"baz") . unwrap () . escapes , 2) ; ParamValue :: try_from_escaped ("\\") . unwrap_err () ; ParamValue :: try_from_escaped ("\"") . unwrap_err () ; ParamValue :: try_from_escaped ("\n") . unwrap_err () ; ParamValue :: try_from_escaped ("\\\n") . unwrap_err () ; } # [test] fn unescape () { assert_eq ! (& ParamValue { escapes : 0 , escaped : "" } . to_unescaped () , "") ; assert_eq ! (& ParamValue { escapes : 0 , escaped : "foo" } . to_unescaped () , "foo") ; assert_eq ! (& ParamValue { escapes : 1 , escaped : "\\foo" } . to_unescaped () , "foo") ; assert_eq ! (& ParamValue { escapes : 1 , escaped : "fo\\o" } . to_unescaped () , "foo") ; assert_eq ! (& ParamValue { escapes : 1 , escaped : "foo\\bar" } . to_unescaped () , "foobar") ; assert_eq ! (& ParamValue { escapes : 3 , escaped : "\\foo\\ba\\r" } . to_unescaped () , "foobar") ; } }
    };
}

tests!()