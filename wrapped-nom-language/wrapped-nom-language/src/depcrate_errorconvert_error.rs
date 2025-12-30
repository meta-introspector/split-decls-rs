// Generated macro for convert_error (function)
macro_rules! Depcrate_errorconvert_error {
() => {
// Module: crate::error
// Provides: {"convert_error"}
// Dependencies: {}
# [doc = " Transforms a `VerboseError` into a trace with input position information"] # [doc = ""] # [doc = " The errors contain references to input data that must come from `input`,"] # [doc = " because nom calculates byte offsets between them"] pub fn convert_error < I : core :: ops :: Deref < Target = str > > (input : I , e : VerboseError < I >) -> String { use nom :: Offset ; use std :: fmt :: Write ; let mut result = String :: new () ; for (i , (substring , kind)) in e . errors . iter () . enumerate () { let offset = input . offset (substring) ; if input . is_empty () { match kind { VerboseErrorKind :: Char (c) => { write ! (& mut result , "{}: expected '{}', got empty input\n\n" , i , c) } VerboseErrorKind :: Context (s) => write ! (& mut result , "{}: in {}, got empty input\n\n" , i , s) , VerboseErrorKind :: Nom (e) => write ! (& mut result , "{}: in {:?}, got empty input\n\n" , i , e) , } } else { let prefix = & input . as_bytes () [.. offset] ; let line_number = prefix . iter () . filter (| & & b | b == b'\n') . count () + 1 ; let line_begin = prefix . iter () . rev () . position (| & b | b == b'\n') . map (| pos | offset - pos) . unwrap_or (0) ; let line = input [line_begin ..] . lines () . next () . unwrap_or (& input [line_begin ..]) . trim_end () ; let column_number = line . offset (substring) + 1 ; match kind { VerboseErrorKind :: Char (c) => { if let Some (actual) = substring . chars () . next () { write ! (& mut result , "{i}: at line {line_number}:\n\
               {line}\n\
               {caret:>column$}\n\
               expected '{expected}', found {actual}\n\n" , i = i , line_number = line_number , line = line , caret = '^' , column = column_number , expected = c , actual = actual ,) } else { write ! (& mut result , "{i}: at line {line_number}:\n\
               {line}\n\
               {caret:>column$}\n\
               expected '{expected}', got end of input\n\n" , i = i , line_number = line_number , line = line , caret = '^' , column = column_number , expected = c ,) } } VerboseErrorKind :: Context (s) => write ! (& mut result , "{i}: at line {line_number}, in {context}:\n\
             {line}\n\
             {caret:>column$}\n\n" , i = i , line_number = line_number , context = s , line = line , caret = '^' , column = column_number ,) , VerboseErrorKind :: Nom (e) => write ! (& mut result , "{i}: at line {line_number}, in {nom_err:?}:\n\
             {line}\n\
             {caret:>column$}\n\n" , i = i , line_number = line_number , nom_err = e , line = line , caret = '^' , column = column_number ,) , } } . unwrap () ; } result }
};
}
