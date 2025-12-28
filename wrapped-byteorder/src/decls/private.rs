macro_rules! deps {
    () => {
        BigEndian!();
        LittleEndian!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { # [doc = " Sealed stops crates other than byteorder from implementing any traits"] # [doc = " that use it."] pub trait Sealed { } impl Sealed for super :: LittleEndian { } impl Sealed for super :: BigEndian { } }
    };
}

private!()