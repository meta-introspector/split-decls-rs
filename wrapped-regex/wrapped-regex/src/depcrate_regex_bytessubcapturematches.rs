// Generated macro for SubCaptureMatches (struct)
macro_rules! Depcrate_regex_bytesSubCaptureMatches {
() => {
// Module: crate::regex::bytes
// Provides: {"SubCaptureMatches"}
// Dependencies: {}
# [doc = " An iterator over all group matches in a [`Captures`] value."] # [doc = ""] # [doc = " This iterator yields values of type `Option<Match<'h>>`, where `'h` is the"] # [doc = " lifetime of the haystack that the matches are for. The order of elements"] # [doc = " yielded corresponds to the order of the opening parenthesis for the group"] # [doc = " in the regex pattern. `None` is yielded for groups that did not participate"] # [doc = " in the match."] # [doc = ""] # [doc = " The first element always corresponds to the implicit group for the overall"] # [doc = " match. Since this iterator is created by a [`Captures`] value, and a"] # [doc = " `Captures` value is only created when a match occurs, it follows that the"] # [doc = " first element yielded by this iterator is guaranteed to be non-`None`."] # [doc = ""] # [doc = " The lifetime `'c` corresponds to the lifetime of the `Captures` value that"] # [doc = " created this iterator, and the lifetime `'h` corresponds to the originally"] # [doc = " matched haystack."] # [derive (Clone , Debug)] pub struct SubCaptureMatches < 'c , 'h > { haystack : & 'h [u8] , it : captures :: CapturesPatternIter < 'c > , }
};
}
