// Generated macro for macro_19 (macro)
macro_rules! Depcrate_generatedmacro_19 {
() => {
// Module: crate::generated
// Provides: {"macro_19"}
// Dependencies: {}
extern_protocol ! (# [doc = " Entry subclasses conforming to this protocol represent"] # [doc = " entries that were made using a handle and a format string."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/oslog/oslogentrywithpayload?language=objc)"] pub unsafe trait OSLogEntryWithPayload { # [doc = " The category from the os_log_t handle used."] # [unsafe (method (category))] # [unsafe (method_family = none)] unsafe fn category (& self) -> Retained < NSString >; # [doc = " An array of the various parts of the composed message."] # [unsafe (method (components))] # [unsafe (method_family = none)] unsafe fn components (& self) -> Retained < NSArray < OSLogMessageComponent >>; # [doc = " The format string used."] # [unsafe (method (formatString))] # [unsafe (method_family = none)] unsafe fn formatString (& self) -> Retained < NSString >; # [doc = " The subsystem of the os_log_t handle used."] # [unsafe (method (subsystem))] # [unsafe (method_family = none)] unsafe fn subsystem (& self) -> Retained < NSString >; }) ;
};
}
