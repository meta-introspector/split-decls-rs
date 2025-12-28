macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ReaderAddress {
    () => {
        deps!();
        # [doc = " A trait for addresses within a DWARF section."] # [doc = ""] # [doc = " Currently this is a simple extension trait for `u64`, but it may be expanded"] # [doc = " in the future to support user-defined address types."] pub (crate) trait ReaderAddress : Sized { # [doc = " Add a length to an address of the given size."] # [doc = ""] # [doc = " Returns an error for overflow."] fn add_sized (self , length : u64 , size : u8) -> Result < Self > ; # [doc = " Add a length to an address of the given size."] # [doc = ""] # [doc = " Wraps the result to the size of the address to allow for the possibility"] # [doc = " that the length is a negative value."] fn wrapping_add_sized (self , length : u64 , size : u8) -> Self ; # [doc = " The all-zeros value of an address."] fn zeros () -> Self ; # [doc = " The all-ones value of an address of the given size."] fn ones_sized (size : u8) -> Self ; # [doc = " Return the minimum value for a tombstone address."] # [doc = ""] # [doc = " A variety of values may be used as tombstones in DWARF data.  DWARF 6 specifies a"] # [doc = " tombstone value of -1, and this is compatible with most sections in earlier DWARF"] # [doc = " versions. However, for .debug_loc and .debug_ranges in DWARF 4 and earlier, the"] # [doc = " tombstone value is -2, because -1 already has a special meaning. -2 has also been"] # [doc = " seen in .debug_line, possibly from a proprietary fork of lld."] # [doc = ""] # [doc = " So this function returns -2 (cast to an unsigned value), and callers can consider"] # [doc = " addresses greater than or equal to this value to be tombstones."] # [doc = ""] # [doc = " Prior to the use of -1 or -2 for tombstones, it was common to use 0 or 1."] # [doc = " Additionally, gold may leave the relocation addend in place. These values are not"] # [doc = " handled by this function, so callers will need to handle them separately if they"] # [doc = " want to."] fn min_tombstone (size : u8) -> Self { Self :: zeros () . wrapping_add_sized (- 2i64 as u64 , size) } }
    };
}

ReaderAddress!()