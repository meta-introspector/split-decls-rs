#[derive(Clone, Debug)]
pub enum ParseResult<F> {
    Success(NamedMatches),
    Failure(F),
    Error(Span, Cow<'static, str>),
    ErrorReported(ErrorGuaranteed),
}
