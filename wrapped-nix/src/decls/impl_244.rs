macro_rules! deps {
    () => {
        PollTimeout!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl PollTimeout { # [doc = " Blocks indefinitely."] # [doc = ""] # [doc = " > Specifying a negative value in timeout means an infinite timeout."] pub const NONE : Self = Self (- 1) ; # [doc = " Returns immediately."] # [doc = ""] # [doc = " > Specifying a timeout of zero causes poll() to return immediately, even if no file"] # [doc = " > descriptors are ready."] pub const ZERO : Self = Self (0) ; # [doc = " Blocks for at most [`i32::MAX`] milliseconds."] pub const MAX : Self = Self (i32 :: MAX) ; # [doc = " Returns if `self` equals [`PollTimeout::NONE`]."] pub fn is_none (& self) -> bool { * self <= Self :: NONE } # [doc = " Returns if `self` does not equal [`PollTimeout::NONE`]."] pub fn is_some (& self) -> bool { ! self . is_none () } # [doc = " Returns the timeout in milliseconds if there is some, otherwise returns `None`."] pub fn as_millis (& self) -> Option < u32 > { self . is_some () . then_some (u32 :: try_from (self . 0) . unwrap ()) } # [doc = " Returns the timeout as a `Duration` if there is some, otherwise returns `None`."] pub fn duration (& self) -> Option < Duration > { self . as_millis () . map (| x | Duration :: from_millis (u64 :: from (x))) } }
    };
}

impl_244!();