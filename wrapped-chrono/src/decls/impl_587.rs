macro_rules! deps {
    () => {
        TimeZoneName!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        impl AsRef < str > for TimeZoneName { fn as_ref (& self) -> & str { unsafe { str :: from_utf8_unchecked (self . as_bytes ()) } } }
    };
}

impl_587!()