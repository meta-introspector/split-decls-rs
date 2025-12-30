// Generated macro for impl_842 (impl)
macro_rules! Depcrate_spanimpl_842 {
() => {
// Module: crate::span
// Provides: {"impl_842"}
// Dependencies: {}
impl UnitSet { # [doc = " Return a bit set representing all units as zero."] # [inline] fn empty () -> UnitSet { UnitSet (0) } # [doc = " Set the given `unit` to `is_zero` status in this set."] # [doc = ""] # [doc = " When `is_zero` is false, the unit is added to this set. Otherwise,"] # [doc = " the unit is removed from this set."] # [inline] fn set (self , unit : Unit , is_zero : bool) -> UnitSet { let bit = 1 << unit as usize ; if is_zero { UnitSet (self . 0 & ! bit) } else { UnitSet (self . 0 | bit) } } # [doc = " Returns true if and only if no units are in this set."] # [inline] pub (crate) fn is_empty (& self) -> bool { self . 0 == 0 } # [doc = " Returns true if and only if this `Span` contains precisely one"] # [doc = " non-zero unit corresponding to the unit given."] # [inline] pub (crate) fn contains_only (self , unit : Unit) -> bool { self . 0 == (1 << unit as usize) } # [doc = " Returns this set, but with only calendar units."] # [inline] pub (crate) fn only_calendar (self) -> UnitSet { UnitSet (self . 0 & 0b0000_0011_1100_0000) } # [doc = " Returns this set, but with only time units."] # [inline] pub (crate) fn only_time (self) -> UnitSet { UnitSet (self . 0 & 0b0000_0000_0011_1111) } # [doc = " Returns the largest unit in this set, or `None` if none are present."] # [inline] pub (crate) fn largest_unit (self) -> Option < Unit > { let zeros = usize :: try_from (self . 0 . leading_zeros ()) . ok () ? ; 15usize . checked_sub (zeros) . and_then (Unit :: from_usize) } }
};
}
