// Generated macro for FastEnumeratorHelper (struct)
macro_rules! Depcrate_iterFastEnumeratorHelper {
() => {
// Module: crate::iter
// Provides: {"FastEnumeratorHelper"}
// Dependencies: {}
# [doc = " Helper type for doing fast enumeration."] # [doc = ""] # [doc = " See the following other implementations of this:"] # [doc = " - [Swift](https://github.com/apple/swift-corelibs-foundation/blob/2d23cf3dc07951ed2b988608d08d7a54cc53b26e/Darwin/Foundation-swiftoverlay/NSFastEnumeration.swift#L23)"] # [doc = " - [Clang](https://github.com/llvm/llvm-project/blob/28d85d207fc37b5593c17a25f687c91b7afda5b4/clang/lib/Frontend/Rewrite/RewriteModernObjC.cpp#L1653-L1850)"] # [derive (Debug , PartialEq)] struct FastEnumeratorHelper { state : NSFastEnumerationState , buf : [* mut AnyObject ; BUF_SIZE] , current_item : usize , items_count : usize , }
};
}
