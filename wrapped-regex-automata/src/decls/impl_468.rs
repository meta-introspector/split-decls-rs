macro_rules! deps {
    () => {
        NFA!();
        WhichCaptures!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl WhichCaptures { # [doc = " Returns true if this configuration indicates that no capture states"] # [doc = " should be produced in an NFA."] pub fn is_none (& self) -> bool { matches ! (* self , WhichCaptures :: None) } # [doc = " Returns true if this configuration indicates that some capture states"] # [doc = " should be added to an NFA. Note that this might only include capture"] # [doc = " states for implicit capture groups."] pub fn is_any (& self) -> bool { ! self . is_none () } }
    };
}

impl_468!();