macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! private {
    () => {
        deps!();
        # [doc = " Sealed traits"] mod private { pub trait Sealed { } }
    };
}

private!();