// Generated macro for impl_790 (impl)
macro_rules! Depcrate_spanimpl_790 {
() => {
// Module: crate::span
// Provides: {"impl_790"}
// Dependencies: {}
impl < 'a > SpanArithmetic < 'a > { # [inline] fn relative < R : Into < SpanRelativeTo < 'a > > > (self , relative : R ,) -> SpanArithmetic < 'a > { SpanArithmetic { relative : Some (relative . into ()) , .. self } } # [inline] fn checked_add (self , span1 : Span) -> Result < Span , Error > { match self . duration . to_signed () ? { SDuration :: Span (span2) => { span1 . checked_add_span (self . relative , & span2) } SDuration :: Absolute (dur2) => { span1 . checked_add_duration (self . relative , dur2) } } } }
};
}
