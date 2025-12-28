macro_rules! deps {
    () => {
        FixedOffset!();
    };
}

macro_rules! Offset {
    () => {
        deps!();
        # [doc = " The offset from the local time to UTC."] pub trait Offset : Sized + Clone + fmt :: Debug { # [doc = " Returns the fixed offset from UTC to the local time stored."] fn fix (& self) -> FixedOffset ; }
    };
}

Offset!();