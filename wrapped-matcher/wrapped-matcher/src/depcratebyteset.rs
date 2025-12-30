// Generated macro for ByteSet (struct)
macro_rules! DepcrateByteSet {
() => {
// Module: crate
// Provides: {"ByteSet"}
// Dependencies: {}
# [doc = " A set of bytes."] # [doc = ""] # [doc = " In this crate, byte sets are used to express bytes that can never appear"] # [doc = " anywhere in a match for a particular implementation of the `Matcher` trait."] # [doc = " Specifically, if such a set can be determined, then it's possible for"] # [doc = " callers to perform additional operations on the basis that certain bytes"] # [doc = " may never match."] # [doc = ""] # [doc = " For example, if a search is configured to possibly produce results that"] # [doc = " span multiple lines but a caller provided pattern can never match across"] # [doc = " multiple lines, then it may make sense to divert to more optimized line"] # [doc = " oriented routines that don't need to handle the multi-line match case."] # [derive (Clone , Debug)] pub struct ByteSet (BitSet) ;
};
}
