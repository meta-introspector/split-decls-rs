// Generated macro for ParseErrorInto (trait)
macro_rules! Depcrate_errorParseErrorInto {
() => {
// Module: crate::error
// Provides: {"ParseErrorInto"}
// Dependencies: {}
# [doc = " Defines a conversion between two parse error types."] # [doc = ""] # [doc = " Like `ParseError::into_other` but with a more general signature"] # [doc = " (This will take the place of `into_other` on breaking release of combine)"] pub trait ParseErrorInto < Item , Range , Position > : Sized { fn into_other_error < T , Item2 , Range2 , Position2 > (self) -> T where T : ParseError < Item2 , Range2 , Position2 > , Item2 : From < Item > , Range2 : From < Range > , Position2 : From < Position > ; }
};
}
