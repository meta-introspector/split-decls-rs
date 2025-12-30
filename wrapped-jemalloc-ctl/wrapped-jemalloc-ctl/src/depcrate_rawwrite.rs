// Generated macro for write (function)
macro_rules! Depcrate_rawwrite {
() => {
// Module: crate::raw
// Provides: {"write"}
// Dependencies: {}
# [doc = " Uses the null-terminated string `name` as the key to the _MALLCTL NAMESPACE_"] # [doc = " and writes it `value`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is `unsafe` because it is possible to use it to construct an"] # [doc = " invalid `T`, for example, by passing `T=u8` for a key expecting `bool`. The"] # [doc = " sizes of `bool` and `u8` match, but `bool` cannot represent all values that"] # [doc = " `u8` can."] pub unsafe fn write < T > (name : & [u8] , mut value : T) -> Result < () > { validate_name (name) ; cvt (tikv_jemalloc_sys :: mallctl (name as * const _ as * const c_char , ptr :: null_mut () , ptr :: null_mut () , & mut value as * mut _ as * mut _ , mem :: size_of :: < T > () ,)) }
};
}
