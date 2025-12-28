macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ReaderOffset {
    () => {
        deps!();
        # [doc = " A trait for offsets with a DWARF section."] # [doc = ""] # [doc = " This allows consumers to choose a size that is appropriate for their address space."] pub trait ReaderOffset : Debug + Copy + Eq + Ord + Hash + Add < Output = Self > + AddAssign + Sub < Output = Self > { # [doc = " Convert a u8 to an offset."] fn from_u8 (offset : u8) -> Self ; # [doc = " Convert a u16 to an offset."] fn from_u16 (offset : u16) -> Self ; # [doc = " Convert an i16 to an offset."] fn from_i16 (offset : i16) -> Self ; # [doc = " Convert a u32 to an offset."] fn from_u32 (offset : u32) -> Self ; # [doc = " Convert a u64 to an offset."] # [doc = ""] # [doc = " Returns `Error::UnsupportedOffset` if the value is too large."] fn from_u64 (offset : u64) -> Result < Self > ; # [doc = " Convert an offset to a u64."] fn into_u64 (self) -> u64 ; # [doc = " Wrapping (modular) addition. Computes `self + other`."] fn wrapping_add (self , other : Self) -> Self ; # [doc = " Checked subtraction. Computes `self - other`."] fn checked_sub (self , other : Self) -> Option < Self > ; }
    };
}

ReaderOffset!();