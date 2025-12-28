macro_rules! deps {
    () => {
        ValidationResult!();
        Result!();
        ServerError!();
    };
}

macro_rules! ValidationFut {
    () => {
        deps!();
        type ValidationFut < 'a > = & 'a mut (dyn Future < Output = Result < ValidationResult , Vec < ServerError > > > + Send + Unpin) ;
    };
}

ValidationFut!()