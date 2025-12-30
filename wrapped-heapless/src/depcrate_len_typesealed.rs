// Generated macro for Sealed (trait)
macro_rules! Depcrate_len_typeSealed {
() => {
// Module: crate::len_type
// Provides: {"Sealed"}
// Dependencies: {}
pub trait Sealed : Send + Sync + Copy + Display + Debug + PartialEq + Add < Output = Self > + AddAssign + Sub < Output = Self > + SubAssign + PartialOrd + TryFrom < usize , Error : Debug > + TryInto < usize , Error : Debug > { # [doc = " The zero value of the integer type."] const ZERO : Self ; # [doc = " The one value of the integer type."] const MAX : Self ; # [doc = " The maximum value of this type, as a `usize`."] const MAX_USIZE : usize ; # [doc = " The one value of the integer type."] # [doc = ""] # [doc = " It's a function instead of constant because we want to have implementation which panics for"] # [doc = " type `ZeroLenType`"] fn one () -> Self ; # [doc = " An infallible conversion from `usize` to `LenT`."] # [inline] fn from_usize (val : usize) -> Self { val . try_into () . unwrap () } # [doc = " An infallible conversion from `LenT` to `usize`."] # [inline] fn into_usize (self) -> usize { self . try_into () . unwrap () } # [doc = " Converts `LenT` into `Some(usize)`, unless it's `Self::MAX`, where it returns `None`."] # [inline] fn to_non_max (self) -> Option < usize > { if self == Self :: MAX { None } else { Some (self . into_usize ()) } } }
};
}
