// Generated macro for tests (module)
macro_rules! Depcrate_types_scalarstests {
() => {
// Module: crate::types::scalars
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { parser :: { ScalarToken , StringLiteral } , value :: { DefaultScalarValue , ParseScalarValue , ScalarValue as _ } , } ; use super :: { EmptyMutation , EmptySubscription , ID } ; # [test] fn test_id_from_string () { let actual = ID :: from (String :: from ("foo")) ; let expected = ID ("foo" . into ()) ; assert_eq ! (actual , expected) ; } # [test] fn test_id_new () { let actual = ID :: new ("foo") ; let expected = ID ("foo" . into ()) ; assert_eq ! (actual , expected) ; } # [test] fn test_id_deref () { let id = ID ("foo" . into ()) ; assert_eq ! (id . len () , 3) ; } # [test] fn test_id_display () { let id = ID ("foo" . into ()) ; assert_eq ! (id . to_string () , "foo") ; } # [test] fn parse_strings () { for (input , expected) in [(r#""simple""# , "simple") , (r#"" white space ""# , " white space ") , (r#""quote \"""# , r#"quote ""#) , (r#""escaped \n\r\b\t\f""# , "escaped \n\r\u{0008}\t\u{000c}") , (r#""slashes \\ \/""# , r"slashes \ /") , (r#""unicode \u1234\u5678\u90AB\uCDEF""# , "unicode \u{1234}\u{5678}\u{90ab}\u{cdef}" ,) , (r#""string with unicode escape outside BMP \u{1F600}""# , "string with unicode escape outside BMP \u{1F600}" ,) , (r#""string with minimal unicode escape \u{0}""# , "string with minimal unicode escape \u{0}" ,) , (r#""string with maximal unicode escape \u{10FFFF}""# , "string with maximal unicode escape \u{10FFFF}" ,) , (r#""string with maximal minimal unicode escape \u{000000}""# , "string with maximal minimal unicode escape \u{000000}" ,) , (r#""string with unicode surrogate pair escape \uD83D\uDE00""# , "string with unicode surrogate pair escape \u{1f600}" ,) , (r#""string with minimal surrogate pair escape \uD800\uDC00""# , "string with minimal surrogate pair escape \u{10000}" ,) , (r#""string with maximal surrogate pair escape \uDBFF\uDFFF""# , "string with maximal surrogate pair escape \u{10FFFF}" ,) ,] { let res = < String as ParseScalarValue < DefaultScalarValue > > :: from_str (ScalarToken :: String (StringLiteral :: Quoted (input)) ,) ; assert ! (res . is_ok () , "parsing error occurred: {}" , res . unwrap_err ()) ; let s : Option < String > = res . unwrap () . try_to () . ok () ; assert ! (s . is_some () , "no string returned") ; assert_eq ! (s . unwrap () , expected) ; } } # [test] fn parse_block_strings () { for (input , expected) in [(r#""""""""# , "") , (r#""""simple""""# , "simple") , (r#"""" white space """"# , " white space ") , (r#""""contains " quote""""# , r#"contains " quote"#) , (r#""""contains \""" triple quote""""# , r#"contains """ triple quote"# ,) , (r#""""contains \"" double quote""""# , r#"contains \"" double quote"# ,) , (r#""""contains \\""" triple quote""""# , r#"contains \""" triple quote"# ,) , (r#""""\"""quote" """"# , r#""""quote" "#) , (r#""""multi\nline""""# , r"multi\nline") , (r#""""multi\rline\r\nnormalized""""# , r"multi\rline\r\nnormalized" ,) , (r#""""unescaped \\n\\r\\b\\t\\f\\u1234""""# , r"unescaped \\n\\r\\b\\t\\f\\u1234" ,) , (r#""""unescaped unicode outside BMP \u{1f600}""""# , r"unescaped unicode outside BMP \u{1f600}" ,) , (r#""""slashes \\\\ \\/""""# , r"slashes \\\\ \\/") , (r#""""

        spans
          multiple
            lines

        """"# , "spans\n  multiple\n    lines" ,) , (r#""""
    Hello,
      World!

    Yours,
      GraphQL.""""# , "Hello,\n  World!\n\nYours,\n  GraphQL." ,) , (r#""""

    Hello,
      World!

    Yours,
      GraphQL.

        """"# , "Hello,\n  World!\n\nYours,\n  GraphQL." ,) , (r#""""    Hello,
      World!

    Yours,
      GraphQL.""""# , "    Hello,\n  World!\n\nYours,\n  GraphQL." ,) , (r#""""
    Hello,
      World!

    Yours,
      GraphQL.   """"# , "Hello,\n  World!\n\nYours,\n  GraphQL.   " ,) ,] { let res = < String as ParseScalarValue < DefaultScalarValue > > :: from_str (ScalarToken :: String (StringLiteral :: Block (input)) ,) ; assert ! (res . is_ok () , "parsing error occurred: {}" , res . unwrap_err ()) ; let s : Option < String > = res . unwrap () . try_to () . ok () ; assert ! (s . is_some () , "no string returned") ; assert_eq ! (s . unwrap () , expected) ; } } # [test] fn parse_f64_from_int () { for (v , expected) in [("0" , 0) , ("128" , 128) , ("1601942400" , 1601942400) , ("1696550400" , 1696550400) , ("-1" , - 1) ,] { let n = < f64 as ParseScalarValue < DefaultScalarValue > > :: from_str (ScalarToken :: Int (v)) ; assert ! (n . is_ok () , "A parsing error occurred: {:?}" , n . unwrap_err ()) ; let n : Option < f64 > = n . unwrap () . try_to () . ok () ; assert ! (n . is_some () , "No `f64` returned") ; assert_eq ! (n . unwrap () , f64 :: from (expected)) ; } } # [test] fn parse_f64_from_float () { for (v , expected) in [("0." , 0.) , ("1.2" , 1.2) , ("1601942400." , 1601942400.) , ("1696550400." , 1696550400.) , ("-1.2" , - 1.2) ,] { let n = < f64 as ParseScalarValue < DefaultScalarValue > > :: from_str (ScalarToken :: Float (v)) ; assert ! (n . is_ok () , "A parsing error occurred: {:?}" , n . unwrap_err ()) ; let n : Option < f64 > = n . unwrap () . try_to () . ok () ; assert ! (n . is_some () , "No `f64` returned") ; assert_eq ! (n . unwrap () , expected) ; } } # [test] fn empty_mutation_is_send () { fn check_if_send < T : Send > () { } check_if_send :: < EmptyMutation < () > > () ; } # [test] fn empty_subscription_is_send () { fn check_if_send < T : Send > () { } check_if_send :: < EmptySubscription < () > > () ; } # [test] fn default_is_invariant_over_type () { struct Bar ; let _ = EmptySubscription :: < Bar > :: default () ; let _ = EmptyMutation :: < Bar > :: default () ; } }
};
}
