// Generated macro for impl_143 (impl)
macro_rules! Depcrateimpl_143 {
() => {
// Module: crate
// Provides: {"impl_143"}
// Dependencies: {}
impl fmt :: Display for ErrorKind { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use ErrorKind :: * ; match * self { DoubleFlagNegation => write ! (f , "Only one negation symbol is allowed in flags.") , DuplicateCaptureName (ref s) => write ! (f , "Capture name '{}' is used more than once." , s) , EmptyAlternate => write ! (f , "Alternations cannot be empty.") , EmptyCaptureName => write ! (f , "Capture names cannot be empty.") , EmptyFlagNegation => write ! (f , "Flag negation requires setting at least one flag.") , EmptyGroup => write ! (f , "Empty regex groups (e.g., '()') are not allowed.") , InvalidBase10 (ref s) => write ! (f , "Not a valid base 10 number: '{}'" , s) , InvalidBase16 (ref s) => write ! (f , "Not a valid base 16 number: '{}'" , s) , InvalidCaptureName (ref s) => write ! (f , "Invalid capture name: '{}'. Capture names must \
                           consist of [_a-zA-Z0-9] and are not allowed to \
                           start with with a number." , s) , InvalidClassRange { start , end } => write ! (f , "Invalid character class range '{}-{}'. \
                           Character class ranges must start with the smaller \
                           character, but {} > {}" , start , end , start , end) , InvalidClassEscape (ref e) => write ! (f , "Invalid escape sequence in character \
                           class: '{}'." , e) , InvalidRepeatRange { min , max } => write ! (f , "Invalid counted repetition range: {{{}, {}}}. \
                           Counted repetition ranges must start with the \
                           minimum, but {} > {}" , min , max , min , max) , InvalidScalarValue (c) => write ! (f , "Number does not correspond to a Unicode scalar \
                           value: '{}'." , c) , MissingBase10 => write ! (f , "Missing maximum in counted reptition operator.") , RepeaterExpectsExpr => write ! (f , "Missing expression for reptition operator.") , RepeaterUnexpectedExpr (ref e) => write ! (f , "Invalid application of reptition operator to: \
                          '{}'." , e) , UnclosedCaptureName (ref s) => write ! (f , "Capture name group for '{}' is not closed. \
                           (Missing a '>'.)" , s) , UnclosedHex => write ! (f , "Unclosed hexadecimal literal (missing a '}}').") , UnclosedParen => write ! (f , "Unclosed parenthesis.") , UnclosedRepeat => write ! (f , "Unclosed counted repetition (missing a '}}').") , UnclosedUnicodeName => write ! (f , "Unclosed Unicode literal (missing a '}}').") , UnexpectedClassEof => write ! (f , "Character class was not closed before the end of \
                           the regex (missing a ']').") , UnexpectedEscapeEof => write ! (f , "Started an escape sequence that didn't finish \
                           before the end of the regex.") , UnexpectedFlagEof => write ! (f , "Inline flag settings was not closed before the end \
                           of the regex (missing a ')' or ':').") , UnexpectedTwoDigitHexEof => write ! (f , "Unexpected end of two digit hexadecimal literal.") , UnopenedParen => write ! (f , "Unopened parenthesis.") , UnrecognizedEscape (c) => write ! (f , "Unrecognized escape sequence: '\\{}'." , c) , UnrecognizedFlag (c) => write ! (f , "Unrecognized flag: '{}'. \
                           (Allowed flags: i, m, s, U, u, x.)" , c) , UnrecognizedUnicodeClass (ref s) => write ! (f , "Unrecognized Unicode class name: '{}'." , s) , StackExhausted => write ! (f , "Exhausted space required to parse regex with too \
                           much nesting.") , FlagNotAllowed (flag) => write ! (f , "Use of the flag '{}' is not allowed." , flag) , UnicodeNotAllowed => write ! (f , "Unicode features are not allowed when the Unicode \
                           (u) flag is not set.") , InvalidUtf8 => write ! (f , "Matching arbitrary bytes is not allowed.") , EmptyClass => write ! (f , "Empty character classes are not allowed.") , __Nonexhaustive => unreachable ! () , } } }
};
}
