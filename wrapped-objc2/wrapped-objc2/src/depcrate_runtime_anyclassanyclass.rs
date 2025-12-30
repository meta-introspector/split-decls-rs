// Generated macro for AnyClass (struct)
macro_rules! Depcrate_runtime_anyclassAnyClass {
() => {
// Module: crate::runtime::anyclass
// Provides: {"AnyClass"}
// Dependencies: {}
# [doc = " An opaque type that represents an Objective-C class."] # [doc = ""] # [doc = " This is an opaque type meant to be used behind a shared reference"] # [doc = " `&AnyClass`, which is semantically equivalent to `Class _Nonnull`."] # [doc = ""] # [doc = " A nullable class can be used as `Option<&AnyClass>`."] # [doc = ""] # [doc = " See [Apple's documentation](https://developer.apple.com/documentation/objectivec/class?language=objc)."] # [repr (C)] # [doc (alias = "Class")] # [doc (alias = "objc_class")] pub struct AnyClass { inner : AnyObject , }
};
}
