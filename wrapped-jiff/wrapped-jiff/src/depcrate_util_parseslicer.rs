// Generated macro for slicer (function)
macro_rules! Depcrate_util_parseslicer {
() => {
// Module: crate::util::parse
// Provides: {"slicer"}
// Dependencies: {}
# [doc = " Returns a function that converts two slices to the slice between them."] # [doc = ""] # [doc = " This takes a starting point as input and returns a function that, when"] # [doc = " given an ending point (greater than or equal to the starting point), it"] # [doc = " returns a slice beginning at the starting point and ending just at the"] # [doc = " ending point."] # [doc = ""] # [doc = " This is useful as a helper function in parsing routines."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if the ending point is not a suffix slice of `start`."] pub (crate) fn slicer < 'a > (start : & 'a [u8] ,) -> impl Fn (& 'a [u8]) -> & 'a [u8] + 'a { let mkoffset = offseter (start) ; move | end | { let offset = mkoffset (end) ; & start [.. offset] } }
};
}
