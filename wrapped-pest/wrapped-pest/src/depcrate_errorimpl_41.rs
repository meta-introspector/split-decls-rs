// Generated macro for impl_41 (impl)
macro_rules! Depcrate_errorimpl_41 {
() => {
// Module: crate::error
// Provides: {"impl_41"}
// Dependencies: {}
impl < R : RuleType > ErrorVariant < R > { # [doc = ""] # [doc = " Returns the error message for [`ErrorVariant`]"] # [doc = ""] # [doc = " If [`ErrorVariant`] is [`CustomError`], it returns a"] # [doc = " [`Cow::Borrowed`] reference to [`message`]. If [`ErrorVariant`] is [`ParsingError`], a"] # [doc = " [`Cow::Owned`] containing \"expected [ErrorVariant::ParsingError::positives] [ErrorVariant::ParsingError::negatives]\" is returned."] # [doc = ""] # [doc = " [`ErrorVariant`]: enum.ErrorVariant.html"] # [doc = " [`CustomError`]: enum.ErrorVariant.html#variant.CustomError"] # [doc = " [`ParsingError`]: enum.ErrorVariant.html#variant.ParsingError"] # [doc = " [`Cow::Owned`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#variant.Owned"] # [doc = " [`Cow::Borrowed`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#variant.Borrowed"] # [doc = " [`message`]: enum.ErrorVariant.html#variant.CustomError.field.message"] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use pest::error::ErrorVariant;"] # [doc = " let variant = ErrorVariant::<()>::CustomError {"] # [doc = "     message: String::from(\"unexpected error\")"] # [doc = " };"] # [doc = ""] # [doc = " println!(\"{}\", variant.message());"] pub fn message (& self) -> Cow < '_ , str > { match self { ErrorVariant :: ParsingError { ref positives , ref negatives , } => Cow :: Owned (Error :: parsing_error_message (positives , negatives , | r | { format ! ("{:?}" , r) })) , ErrorVariant :: CustomError { ref message } => Cow :: Borrowed (message) , } } }
};
}
