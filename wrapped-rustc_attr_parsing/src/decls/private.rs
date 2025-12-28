macro_rules! deps {
    () => {
        Early!();
        Late!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { pub trait Sealed { } impl Sealed for super :: Early { } impl Sealed for super :: Late { } }
    };
}

private!();