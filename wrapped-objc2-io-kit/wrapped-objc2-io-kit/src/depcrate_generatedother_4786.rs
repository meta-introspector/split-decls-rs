// Generated macro for other_4786 (other)
macro_rules! Depcrate_generatedother_4786 {
() => {
// Module: crate::generated
// Provides: {"other_4786"}
// Dependencies: {}
extern "C" { # [doc = " The default mach port used to initiate communication with IOKit."] # [doc = ""] # [doc = " When specifying a main port to IOKit functions, the NULL argument indicates \"use the default\". This is a synonym for NULL, if you'd rather use a named constant."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/kiomainportdefault?language=objc)"] # [cfg (feature = "libc")] pub static kIOMainPortDefault : libc :: mach_port_t ; }
};
}
