// Generated macro for InternalInternal (enum)
macro_rules! Depcrate_formatInternalInternal {
() => {
// Module: crate::format
// Provides: {"InternalInternal"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] enum InternalInternal { # [doc = " Same as [`TimezoneOffsetColonZ`](#variant.TimezoneOffsetColonZ), but"] # [doc = " allows missing minutes (per [ISO 8601][iso8601])."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If you try to use this for printing."] # [doc = ""] # [doc = " [iso8601]: https://en.wikipedia.org/wiki/ISO_8601#Time_offsets_from_UTC"] TimezoneOffsetPermissive , # [doc = " Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 3 and there is no leading dot."] Nanosecond3NoDot , # [doc = " Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 6 and there is no leading dot."] Nanosecond6NoDot , # [doc = " Same as [`Nanosecond`](#variant.Nanosecond) but the accuracy is fixed to 9 and there is no leading dot."] Nanosecond9NoDot , }
};
}
