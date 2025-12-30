// Generated macro for other_357 (other)
macro_rules! Depcrate_numberother_357 {
() => {
// Module: crate::number
// Provides: {"other_357"}
// Dependencies: {}
unsafe extern "C" { pub static kCFBooleanTrue : CFBooleanRef ; pub static kCFBooleanFalse : CFBooleanRef ; pub static kCFNumberPositiveInfinity : CFNumberRef ; pub static kCFNumberNegativeInfinity : CFNumberRef ; pub static kCFNumberNaN : CFNumberRef ; pub fn CFNumberCreate (allocator : CFAllocatorRef , theType : CFNumberType , valuePtr : * const c_void ,) -> CFNumberRef ; pub fn CFNumberGetByteSize (number : CFNumberRef) -> CFIndex ; pub fn CFNumberGetType (number : CFNumberRef) -> CFNumberType ; pub fn CFNumberGetValue (number : CFNumberRef , theType : CFNumberType , valuePtr : * mut c_void ,) -> bool ; pub fn CFNumberIsFloatType (number : CFNumberRef) -> Boolean ; pub fn CFNumberCompare (date : CFNumberRef , other : CFNumberRef , context : * mut c_void ,) -> CFComparisonResult ; pub fn CFNumberGetTypeID () -> CFTypeID ; pub fn CFBooleanGetValue (boolean : CFBooleanRef) -> bool ; pub fn CFBooleanGetTypeID () -> CFTypeID ; }
};
}
