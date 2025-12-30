// Generated macro for decay_parameter_encoding (function)
macro_rules! Depcrate_encodedecay_parameter_encoding {
() => {
// Module: crate::encode
// Provides: {"decay_parameter_encoding"}
// Dependencies: {}
# [doc = " Handle array -> pointer decay."] # [doc = ""] # [doc = " A C method like:"] # [doc = " ```c"] # [doc = " void foo(uint8_t[10] arr);"] # [doc = " ```"] # [doc = ""] # [doc = " Actually has the following ABI;"] # [doc = " ```c"] # [doc = " void foo(uint8_t* arr);"] # [doc = " ```"] # [doc = ""] # [doc = " Whereas the equivalent Rust function:"] # [doc = " ```"] # [doc = " extern \"C-unwind\" {"] # [doc = "     fn foo(arr: [u8; 10]);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Doesn't have a stable ABI (it might choose to pass a pointer, it might"] # [doc = " choose to inline)."] # [doc = ""] # [doc = " This happens through typedefs too, so when auto-generating these, it's"] # [doc = " kinda hard to know what to output."] # [doc = ""] # [doc = " To handle this, we'd like to have the Rust signature be `*const [u8; 10]`,"] # [doc = " as that still carries the bounds information, while still accurately"] # [doc = " describing the ABI."] # [doc = ""] # [doc = " That type has an encoding of `Pointer(Array(10, u8))` though, which is"] # [doc = " correct if it were used in e.g. a struct field, but it isn't when used"] # [doc = " here in a function argument. So we decay the encoding parameter."] const fn decay_parameter_encoding (enc : Encoding) -> Encoding { match enc { Encoding :: Pointer (Encoding :: Array (len , ty)) => Encoding :: Array (* len , ty) , enc => enc , } }
};
}
