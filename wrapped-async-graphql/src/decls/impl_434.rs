macro_rules! deps {
    () => {
        FieldValue!();
        FieldFuture!();
        Result!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl < 'a > FieldFuture < 'a > { # [doc = " Create a `FieldFuture` from a `Future`"] pub fn new < Fut , R > (future : Fut) -> Self where Fut : Future < Output = Result < Option < R > > > + Send + 'a , R : Into < FieldValue < 'a > > + Send , { FieldFuture :: Future (async move { let res = future . await ? ; Ok (res . map (Into :: into)) } . boxed () ,) } # [doc = " Create a `FieldFuture` from a `Value`"] pub fn from_value (value : Option < Value >) -> Self { FieldFuture :: Value (value . map (FieldValue :: from)) } }
    };
}

impl_434!();