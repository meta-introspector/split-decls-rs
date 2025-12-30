// Generated macro for offseter (function)
macro_rules! Depcrate_util_parseoffseter {
() => {
// Module: crate::util::parse
// Provides: {"offseter"}
// Dependencies: {}
# [doc = " Returns a function that converts two slices to an offset."] # [doc = ""] # [doc = " It takes the starting point as input and returns a function that, when"] # [doc = " given an ending point (greater than or equal to the starting point), then"] # [doc = " the corresponding pointers are subtracted and an offset relative to the"] # [doc = " starting point is returned."] # [doc = ""] # [doc = " This is useful as a helper function in parsing routines that use slices"] # [doc = " but want to report offsets."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if the ending point is not a suffix slice of `start`."] pub (crate) fn offseter < 'a > (start : & 'a [u8] ,) -> impl Fn (& 'a [u8]) -> usize + 'a { move | end | (end . as_ptr () as usize) - (start . as_ptr () as usize) }
};
}
