macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! private_try_stream {
    () => {
        deps!();
        mod private_try_stream { use super :: Stream ; pub trait Sealed { } impl < S , T , E > Sealed for S where S : ? Sized + Stream < Item = Result < T , E > > { } }
    };
}

private_try_stream!();