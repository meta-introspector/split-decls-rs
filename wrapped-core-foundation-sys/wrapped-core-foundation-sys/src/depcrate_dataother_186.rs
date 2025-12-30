// Generated macro for other_186 (other)
macro_rules! Depcrate_dataother_186 {
() => {
// Module: crate::data
// Provides: {"other_186"}
// Dependencies: {}
unsafe extern "C" { pub fn CFDataCreate (allocator : CFAllocatorRef , bytes : * const u8 , length : CFIndex) -> CFDataRef ; pub fn CFDataCreateCopy (allocator : CFAllocatorRef , theData : CFDataRef) -> CFDataRef ; pub fn CFDataCreateWithBytesNoCopy (allocator : CFAllocatorRef , bytes : * const u8 , length : CFIndex , bytesDeallocator : CFAllocatorRef ,) -> CFDataRef ; pub fn CFDataGetBytePtr (theData : CFDataRef) -> * const u8 ; pub fn CFDataGetBytes (theData : CFDataRef , range : CFRange , buffer : * mut u8) ; pub fn CFDataGetLength (theData : CFDataRef) -> CFIndex ; pub fn CFDataFind (theData : CFDataRef , dataToFind : CFDataRef , searchRange : CFRange , compareOptions : CFDataSearchFlags ,) -> CFRange ; pub fn CFDataGetTypeID () -> CFTypeID ; pub fn CFDataCreateMutable (allocator : CFAllocatorRef , capacity : CFIndex) -> CFMutableDataRef ; pub fn CFDataCreateMutableCopy (allocator : CFAllocatorRef , capacity : CFIndex , theData : CFDataRef ,) -> CFMutableDataRef ; pub fn CFDataGetMutableBytePtr (theData : CFMutableDataRef) -> * mut u8 ; pub fn CFDataAppendBytes (theData : CFMutableDataRef , bytes : * const u8 , length : CFIndex) ; pub fn CFDataDeleteBytes (theData : CFMutableDataRef , range : CFRange) ; pub fn CFDataReplaceBytes (theData : CFMutableDataRef , range : CFRange , newBytes : * const u8 , newLength : CFIndex ,) ; pub fn CFDataIncreaseLength (theData : CFMutableDataRef , extraLength : CFIndex) ; pub fn CFDataSetLength (theData : CFMutableDataRef , length : CFIndex) ; }
};
}
