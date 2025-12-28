macro_rules! deps {
    () => {
        ServerError!();
        ValidationResult!();
        Result!();
    };
}

macro_rules! ValidationFut {
    () => {
        deps!();
        type ValidationFut < 'a > = & 'a mut (dyn Future < Output = Result < ValidationResult , Vec < ServerError > > > + Send + Unpin) ;
    };
}

ValidationFut!();