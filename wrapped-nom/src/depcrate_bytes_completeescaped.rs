// Generated macro for escaped (function)
macro_rules! Depcrate_bytes_completeescaped {
() => {
// Module: crate::bytes::complete
// Provides: {"escaped"}
// Dependencies: {}
# [doc = " Matches a byte string with escaped characters."] # [doc = ""] # [doc = " * The first argument matches the normal characters (it must not accept the control character)"] # [doc = " * The second argument is the control character (like `\\` in most languages)"] # [doc = " * The third argument matches the escaped characters"] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nom::{Err, error::ErrorKind, Needed, IResult};"] # [doc = " # use nom::character::complete::digit1;"] # [doc = " use nom::bytes::complete::escaped;"] # [doc = " use nom::character::complete::one_of;"] # [doc = ""] # [doc = " fn esc(s: &str) -> IResult<&str, &str> {"] # [doc = "   escaped(digit1, '\\\\', one_of(r#\"\"n\\\"#))(s)"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(esc(\"123;\"), Ok((\";\", \"123\")));"] # [doc = " assert_eq!(esc(r#\"12\\\"34;\"#), Ok((\";\", r#\"12\\\"34\"#)));"] # [doc = " ```"] # [doc = ""] pub fn escaped < 'a , I , Error , F , G > (normal : F , control_char : char , escapable : G ,) -> impl FnMut (I) -> IResult < I , I , Error > where I : Clone + crate :: traits :: Offset + Input + 'a , < I as Input > :: Item : crate :: traits :: AsChar , F : Parser < I , Error = Error > , G : Parser < I , Error = Error > , Error : ParseError < I > , { let mut parser = super :: escaped (normal , control_char , escapable) ; move | i : I | parser . process :: < OutputM < Emit , Emit , Complete > > (i) }
};
}
