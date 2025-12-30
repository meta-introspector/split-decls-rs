// Generated macro for other_4808 (other)
macro_rules! Depcrate_generatedother_4808 {
() => {
// Module: crate::generated
// Provides: {"other_4808"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Match an IOService objects with matching dictionary."] # [doc = ""] # [doc = " This function calls the matching method of an IOService object and returns the boolean result."] # [doc = ""] # [doc = " Parameter `service`: The IOService object to match."] # [doc = ""] # [doc = " Parameter `matching`: A CF dictionary containing matching information. IOKitLib can construct matching dictionaries for common criteria with helper functions such as IOServiceMatching, IOServiceNameMatching, IOBSDNameMatching."] # [doc = ""] # [doc = " Parameter `matches`: The boolean result is returned."] # [doc = ""] # [doc = " Returns: A kern_return_t error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `matching` generic must be of the correct type."] # [doc = " - `matching` generic must be of the correct type."] # [doc = " - `matching` might not allow `None`."] # [doc = " - `matches` must be a valid pointer."] # [cfg (feature = "libc")] pub fn IOServiceMatchPropertyTable (service : io_service_t , matching : Option < & CFDictionary > , matches : * mut libc :: boolean_t ,) -> libc :: kern_return_t ; }
};
}
