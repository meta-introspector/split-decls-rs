macro_rules! deps {
    () => {
        StripStream!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < S > StripStream < S > where S : std :: io :: Write , { # [doc = " Only pass printable data to the inner `Write`"] # [inline] pub fn new (raw : S) -> Self { Self { raw , state : Default :: default () , } } # [doc = " Get the wrapped [`std::io::Write`]"] # [inline] pub fn into_inner (self) -> S { self . raw } # [doc = " Get the wrapped [`std::io::Write`]"] # [inline] pub fn as_inner (& self) -> & S { & self . raw } }
    };
}

impl_113!()