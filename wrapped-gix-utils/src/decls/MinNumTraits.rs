macro_rules! MinNumTraits {
    () => {
        # [doc = " minimal subset of traits used by [`to_signed_with_radix`] and [`to_unsigned_with_radix`]"] pub trait MinNumTraits : Sized + Copy + TryFrom < u32 > { # [doc = " the 0 value for this type"] const ZERO : Self ; # [doc = " convert from a unsigned 32-bit word"] fn from_u32 (n : u32) -> Option < Self > { Self :: try_from (n) . ok () } # [doc = " the checked multiplication operation for this type"] fn checked_mul (self , rhs : Self) -> Option < Self > ; # [doc = " the chekced addition operation for this type"] fn checked_add (self , rhs : Self) -> Option < Self > ; # [doc = " the checked subtraction operation for this type"] fn checked_sub (self , v : Self) -> Option < Self > ; }
    };
}

MinNumTraits!()