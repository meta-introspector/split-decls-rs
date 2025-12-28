macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl PotentialUtf8 { # [doc = " Create a [`PotentialUtf8`] from a byte slice."] # [inline] pub const fn from_bytes (other : & [u8]) -> & Self { unsafe { core :: mem :: transmute (other) } } # [doc = " Create a [`PotentialUtf8`] from a string slice."] # [inline] pub const fn from_str (s : & str) -> & Self { Self :: from_bytes (s . as_bytes ()) } # [doc = " Create a [`PotentialUtf8`] from boxed bytes."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [inline] # [cfg (feature = "alloc")] pub fn from_boxed_bytes (other : Box < [u8] >) -> Box < Self > { unsafe { core :: mem :: transmute (other) } } # [doc = " Create a [`PotentialUtf8`] from a boxed `str`."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [inline] # [cfg (feature = "alloc")] pub fn from_boxed_str (other : Box < str >) -> Box < Self > { Self :: from_boxed_bytes (other . into_boxed_bytes ()) } # [doc = " Get the bytes from a [`PotentialUtf8]."] # [inline] pub const fn as_bytes (& self) -> & [u8] { & self . 0 } # [doc = " Attempt to convert a [`PotentialUtf8`] to a `str`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use potential_utf::PotentialUtf8;"] # [doc = ""] # [doc = " static A: &PotentialUtf8 = PotentialUtf8::from_bytes(b\"abc\");"] # [doc = ""] # [doc = " let b = A.try_as_str().unwrap();"] # [doc = " assert_eq!(b, \"abc\");"] # [doc = " ```"] # [inline] pub fn try_as_str (& self) -> Result < & str , core :: str :: Utf8Error > { core :: str :: from_utf8 (& self . 0) } }
    };
}

impl_23!();