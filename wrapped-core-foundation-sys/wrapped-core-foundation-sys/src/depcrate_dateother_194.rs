// Generated macro for other_194 (other)
macro_rules! Depcrate_dateother_194 {
() => {
// Module: crate::date
// Provides: {"other_194"}
// Dependencies: {}
unsafe extern "C" { pub static kCFAbsoluteTimeIntervalSince1904 : CFTimeInterval ; pub static kCFAbsoluteTimeIntervalSince1970 : CFTimeInterval ; pub fn CFAbsoluteTimeGetCurrent () -> CFAbsoluteTime ; pub fn CFDateCreate (allocator : CFAllocatorRef , at : CFAbsoluteTime) -> CFDateRef ; pub fn CFDateGetAbsoluteTime (date : CFDateRef) -> CFAbsoluteTime ; pub fn CFDateGetTimeIntervalSinceDate (date : CFDateRef , other : CFDateRef) -> CFTimeInterval ; pub fn CFDateCompare (date : CFDateRef , other : CFDateRef , context : * mut c_void ,) -> CFComparisonResult ; pub fn CFDateGetTypeID () -> CFTypeID ; }
};
}
