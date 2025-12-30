// Generated macro for impl_811 (impl)
macro_rules! Depcrate_spanimpl_811 {
() => {
// Module: crate::span
// Provides: {"impl_811"}
// Dependencies: {}
impl < 'a > SpanCompare < 'a > { # [inline] fn new (span : Span) -> SpanCompare < 'static > { SpanCompare { span , relative : None } } # [inline] fn relative < R : Into < SpanRelativeTo < 'a > > > (self , relative : R ,) -> SpanCompare < 'a > { SpanCompare { relative : Some (relative . into ()) , .. self } } fn compare (self , span : Span) -> Result < Ordering , Error > { let (span1 , span2) = (span , self . span) ; let unit = span1 . largest_unit () . max (span2 . largest_unit ()) ; let start = match self . relative { Some (r) => match r . to_relative (unit) ? { Some (r) => r , None => { let nanos1 = span1 . to_invariant_nanoseconds () ; let nanos2 = span2 . to_invariant_nanoseconds () ; return Ok (nanos1 . cmp (& nanos2)) ; } } , None => { requires_relative_date_err (unit) ? ; let nanos1 = span1 . to_invariant_nanoseconds () ; let nanos2 = span2 . to_invariant_nanoseconds () ; return Ok (nanos1 . cmp (& nanos2)) ; } } ; let end1 = start . checked_add (span1) ? . to_nanosecond () ; let end2 = start . checked_add (span2) ? . to_nanosecond () ; Ok (end1 . cmp (& end2)) } }
};
}
