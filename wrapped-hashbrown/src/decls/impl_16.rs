macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Tag { # [doc = " Control tag value for an empty bucket."] pub (crate) const EMPTY : Tag = Tag (0b1111_1111) ; # [doc = " Control tag value for a deleted bucket."] pub (crate) const DELETED : Tag = Tag (0b1000_0000) ; # [doc = " Checks whether a control tag represents a full bucket (top bit is clear)."] # [inline] pub (crate) const fn is_full (self) -> bool { self . 0 & 0x80 == 0 } # [doc = " Checks whether a control tag represents a special value (top bit is set)."] # [inline] pub (crate) const fn is_special (self) -> bool { self . 0 & 0x80 != 0 } # [doc = " Checks whether a special control value is EMPTY (just check 1 bit)."] # [inline] pub (crate) const fn special_is_empty (self) -> bool { debug_assert ! (self . is_special ()) ; self . 0 & 0x01 != 0 } # [doc = " Creates a control tag representing a full bucket with the given hash."] # [inline] # [allow (clippy :: cast_possible_truncation)] pub (crate) const fn full (hash : u64) -> Tag { const MIN_HASH_LEN : usize = if mem :: size_of :: < usize > () < mem :: size_of :: < u64 > () { mem :: size_of :: < usize > () } else { mem :: size_of :: < u64 > () } ; let top7 = hash >> (MIN_HASH_LEN * 8 - 7) ; Tag ((top7 & 0x7f) as u8) } }
    };
}

impl_16!();