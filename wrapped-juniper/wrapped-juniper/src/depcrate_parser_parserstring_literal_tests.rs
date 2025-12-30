// Generated macro for string_literal_tests (module)
macro_rules! Depcrate_parser_parserstring_literal_tests {
() => {
// Module: crate::parser::parser
// Provides: {"string_literal_tests"}
// Dependencies: {}
# [cfg (test)] mod string_literal_tests { use super :: StringLiteral ; # [test] fn quoted () { for (input , expected) in [(r#""""# , "") , (r#""simple""# , "simple") , (r#"" white space ""# , " white space ") , (r#""quote \"""# , r#"quote ""#) , (r#""escaped \n\r\b\t\f""# , "escaped \n\r\u{0008}\t\u{000c}") , (r#""slashes \\ \/""# , r"slashes \ /") , (r#""unicode \u1234\u5678\u90AB\uCDEF""# , "unicode \u{1234}\u{5678}\u{90ab}\u{cdef}" ,) , (r#""string with unicode escape outside BMP \u{1F600}""# , "string with unicode escape outside BMP \u{1F600}" ,) , (r#""string with minimal unicode escape \u{0}""# , "string with minimal unicode escape \u{0}" ,) , (r#""string with maximal unicode escape \u{10FFFF}""# , "string with maximal unicode escape \u{10FFFF}" ,) , (r#""string with maximal minimal unicode escape \u{000000}""# , "string with maximal minimal unicode escape \u{000000}" ,) , (r#""string with unicode surrogate pair escape \uD83D\uDE00""# , "string with unicode surrogate pair escape \u{1f600}" ,) , (r#""string with minimal surrogate pair escape \uD800\uDC00""# , "string with minimal surrogate pair escape \u{10000}" ,) , (r#""string with maximal surrogate pair escape \uDBFF\uDFFF""# , "string with maximal surrogate pair escape \u{10FFFF}" ,) ,] { let res = StringLiteral :: Quoted (input) . parse () ; assert ! (res . is_ok () , "parsing error occurred on {input}: {}" , res . unwrap_err () ,) ; assert_eq ! (res . unwrap () , expected) ; } } # [test] fn quoted_errors () { for (input , expected) in [(r#""bad surrogate \uDEAD""# , r#"Unknown escape sequence "\uDEAD" in string"# ,) , (r#""bad low surrogate pair \uD800\uD800""# , r#"Unknown escape sequence "\uD800" in string"# ,) ,] { let res = StringLiteral :: Quoted (input) . parse () ; assert ! (res . is_err () , "parsing error doesn't occur on {input}") ; let err = res . unwrap_err () ; assert ! (err . to_string () . contains (expected) , "returned error `{err}` doesn't contain `{expected}`" ,) ; } } # [test] fn block () { for (input , expected) in [(r#""""""""# , "") , (r#""""simple""""# , "simple") , (r#"""" white space """"# , " white space ") , (r#""""contains " quote""""# , r#"contains " quote"#) , (r#""""contains \""" triple quote""""# , r#"contains """ triple quote"# ,) , (r#""""contains \"" double quote""""# , r#"contains \"" double quote"# ,) , (r#""""contains \\""" triple quote""""# , r#"contains \""" triple quote"# ,) , (r#""""\"""quote" """"# , r#""""quote" "#) , (r#""""multi\nline""""# , r"multi\nline") , (r#""""multi\rline\r\nnormalized""""# , r"multi\rline\r\nnormalized" ,) , (r#""""unescaped \\n\\r\\b\\t\\f\\u1234""""# , r"unescaped \\n\\r\\b\\t\\f\\u1234" ,) , (r#""""unescaped unicode outside BMP \u{1f600}""""# , r"unescaped unicode outside BMP \u{1f600}" ,) , (r#""""slashes \\\\ \\/""""# , r"slashes \\\\ \\/") , (r#""""

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
      GraphQL.   """"# , "Hello,\n  World!\n\nYours,\n  GraphQL.   " ,) ,] { let res = StringLiteral :: Block (input) . parse () ; assert ! (res . is_ok () , "parsing error occurred on {input}: {}" , res . unwrap_err () ,) ; assert_eq ! (res . unwrap () , expected) ; } } }
};
}
