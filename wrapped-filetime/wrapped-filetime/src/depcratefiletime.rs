// Generated macro for FileTime (struct)
macro_rules! DepcrateFileTime {
() => {
// Module: crate
// Provides: {"FileTime"}
// Dependencies: {}
# [doc = " A helper structure to represent a timestamp for a file."] # [doc = ""] # [doc = " The actual value contined within is platform-specific and does not have the"] # [doc = " same meaning across platforms, but comparisons and stringification can be"] # [doc = " significant among the same platform."] # [derive (Eq , PartialEq , Ord , PartialOrd , Debug , Copy , Clone , Hash)] pub struct FileTime { seconds : i64 , nanos : u32 , }
};
}
