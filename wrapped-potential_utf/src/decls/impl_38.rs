macro_rules! deps {
    () => {
        PotentialUtf16!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl PotentialUtf16 { # [doc = " Create a [`PotentialUtf16`] from a u16 slice."] # [inline] pub const fn from_slice (other : & [u16]) -> & Self { unsafe { core :: mem :: transmute (other) } } pub fn chars (& self) -> impl Iterator < Item = char > + '_ { char :: decode_utf16 (self . 0 . iter () . copied ()) . map (| c | c . unwrap_or (char :: REPLACEMENT_CHARACTER)) } }
    };
}

impl_38!()