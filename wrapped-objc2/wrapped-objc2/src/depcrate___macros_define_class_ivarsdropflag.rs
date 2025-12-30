// Generated macro for DropFlag (enum)
macro_rules! Depcrate___macros_define_class_ivarsDropFlag {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"DropFlag"}
// Dependencies: {}
# [doc = " A type representing the drop flags that may be set for a type."] # [repr (u8)] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub (crate) enum DropFlag { # [doc = " Set to zero to ensure that this is the default when created by the"] # [doc = " Objective-C runtime."] # [doc = ""] # [doc = " Ivars are [documented][obj-init-zeroed] to be zero-initialized after"] # [doc = " allocation, and that has been true since at least [the Objective-C"] # [doc = " version shipped with Mac OS X 10.0][objc4-208-init]."] # [doc = ""] # [doc = " [obj-init-zeroed]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ProgrammingWithObjectiveC/WorkingwithObjects/WorkingwithObjects.html#//apple_ref/doc/uid/TP40011210-CH4-SW7"] # [doc = " [objc4-208-init]: https://github.com/apple-oss-distributions/objc4/blob/objc4-208/runtime/objc-class.m#L367"] # [allow (dead_code)] Allocated = 0x00 , # [doc = " Used when `mem::needs_drop::<T::Ivars>()`, or with debug assertions enabled."] InitializedIvars = 0x0f , # [doc = " Used when `mem::needs_drop::<T>()`, or with debug assertions enabled."] Finalized = 0xff , }
};
}
