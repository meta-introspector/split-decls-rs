macro_rules! deps {
    () => {
        RawEnum!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl < T : TryFrom < u32 > > RawEnum < T > { # [track_caller] pub (crate) fn to_rust (self) -> T where T :: Error : Debug , { T :: try_from (self . value) . expect ("enum value returned by LLVM should be known") } }
    };
}

impl_429!();