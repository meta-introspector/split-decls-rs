// Generated macro for Bool (struct)
macro_rules! Depcrate_runtime_boolBool {
() => {
// Module: crate::runtime::bool
// Provides: {"Bool"}
// Dependencies: {}
# [doc = " The Objective-C `BOOL` type."] # [doc = ""] # [doc = " The type of `BOOL` varies across platforms, so we expose this wrapper. It"] # [doc = " is intended that you convert this into a Rust [`bool`] with the"] # [doc = " [`Bool::as_bool`] method as soon as possible."] # [doc = ""] # [doc = " This is FFI-safe and can be used directly with `msg_send!` and `extern`"] # [doc = " functions as a substitute for `BOOL` in Objective-C. If your Objective-C"] # [doc = " code uses C99 `_Bool`, you should use a `#[repr(transparent)]` wrapper"] # [doc = " around `bool` instead."] # [doc = ""] # [doc = " Note that this is able to contain more states than `bool` on some"] # [doc = " platforms, but these cases should not be relied on! The comparison traits"] # [doc = " `PartialEq`, `PartialOrd` etc. will completely ignore these states."] # [doc = ""] # [doc = " See also the [corresponding documentation entry][docs]."] # [doc = ""] # [doc = " [docs]: https://developer.apple.com/documentation/objectivec/bool?language=objc"] # [repr (transparent)] # [derive (Copy , Clone , Default)] pub struct Bool { value : inner :: BOOL , }
};
}
