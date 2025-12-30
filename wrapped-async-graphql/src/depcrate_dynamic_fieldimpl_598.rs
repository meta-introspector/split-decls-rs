// Generated macro for impl_598 (impl)
macro_rules! Depcrate_dynamic_fieldimpl_598 {
() => {
// Module: crate::dynamic::field
// Provides: {"impl_598"}
// Dependencies: {}
impl < 'a > FieldFuture < 'a > { # [doc = " Create a `FieldFuture` from a `Future`"] pub fn new < Fut , R > (future : Fut) -> Self where Fut : Future < Output = Result < Option < R > > > + Send + 'a , R : Into < FieldValue < 'a > > + Send , { FieldFuture :: Future (async move { let res = future . await ? ; Ok (res . map (Into :: into)) } . boxed () ,) } # [doc = " Create a `FieldFuture` from a `Value`"] pub fn from_value (value : Option < Value >) -> Self { FieldFuture :: Value (value . map (FieldValue :: from)) } }
};
}
