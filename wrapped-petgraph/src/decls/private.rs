macro_rules! deps {
    () => {
        NotZero!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { pub trait Sealed { } impl < T > Sealed for super :: NotZero < T > { } impl < T > Sealed for Option < T > { } }
    };
}

private!()