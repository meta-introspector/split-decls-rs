macro_rules! deps {
    () => {
        Result!();
        StdError!();
    };
}

macro_rules! private {
    () => {
        deps!();
        pub (crate) mod private { use super :: * ; pub trait Sealed { } impl < T , E > Sealed for Result < T , E > where E : ext :: StdError { } impl < T > Sealed for Option < T > { } }
    };
}

private!()