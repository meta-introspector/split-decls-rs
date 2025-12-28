macro_rules! deps {
    () => {
        LittleEndian!();
        BigEndian!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { # [doc = " Sealed stops crates other than byteorder from implementing any traits"] # [doc = " that use it."] pub trait Sealed { } impl Sealed for super :: LittleEndian { } impl Sealed for super :: BigEndian { } }
    };
}

private!()