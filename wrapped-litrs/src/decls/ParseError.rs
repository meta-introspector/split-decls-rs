macro_rules! deps {
    () => {
        ParseErrorKind!();
    };
}

macro_rules! ParseError {
    () => {
        deps!();
        # [doc = " Errors during parsing."] # [doc = ""] # [doc = " This type should be seen primarily for error reporting and not for catching"] # [doc = " specific cases. The span and error kind are not guaranteed to be stable"] # [doc = " over different versions of this library, meaning that a returned error can"] # [doc = " change from one version to the next. There are simply too many fringe cases"] # [doc = " that are not easy to classify as a specific error kind. It depends entirely"] # [doc = " on the specific parser code how an invalid input is categorized."] # [doc = ""] # [doc = " Consider these examples:"] # [doc = " - `'\\` can be seen as"] # [doc = "     - invalid escape in character literal, or"] # [doc = "     - unterminated character literal."] # [doc = " - `'''` can be seen as"] # [doc = "     - empty character literal, or"] # [doc = "     - unescaped quote character in character literal."] # [doc = " - `0b64` can be seen as"] # [doc = "     - binary integer literal with invalid digit 6, or"] # [doc = "     - binary integer literal with invalid digit 4, or"] # [doc = "     - decimal integer literal with invalid digit b, or"] # [doc = "     - decimal integer literal 0 with unknown type suffix `b64`."] # [doc = ""] # [doc = " If you want to see more if these examples, feel free to check out the unit"] # [doc = " tests of this library."] # [doc = ""] # [doc = " While this library does its best to emit sensible and precise errors, and to"] # [doc = " keep the returned errors as stable as possible, full stability cannot be"] # [doc = " guaranteed."] # [derive (Debug , Clone)] pub struct ParseError { pub (crate) span : Option < Range < usize > > , pub (crate) kind : ParseErrorKind , }
    };
}

ParseError!()