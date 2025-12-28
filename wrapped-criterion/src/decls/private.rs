macro_rules! deps {
    () => {
        BenchmarkId!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { pub trait Sealed { } impl Sealed for super :: BenchmarkId { } impl < S : Into < String > > Sealed for S { } }
    };
}

private!()