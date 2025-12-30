// Generated macro for read (function)
macro_rules! Depcrate_rawread {
() => {
// Module: crate::raw
// Provides: {"read"}
// Dependencies: {}
# [doc = " Uses the null-terminated string `name` as key to the _MALLCTL NAMESPACE_ and"] # [doc = " reads its value."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is `unsafe` because it is possible to use it to construct an"] # [doc = " invalid `T`, for example, by passing `T=bool` for a key returning `u8`. The"] # [doc = " sizes of `bool` and `u8` match, but `bool` cannot represent all values that"] # [doc = " `u8` can."] pub unsafe fn read < T : Copy > (name : & [u8]) -> Result < T > { validate_name (name) ; let mut value = MaybeUninit { init : () } ; let mut len = mem :: size_of :: < T > () ; cvt (tikv_jemalloc_sys :: mallctl (name as * const _ as * const c_char , & mut value . init as * mut _ as * mut _ , & mut len , ptr :: null_mut () , 0 ,)) ? ; assert_eq ! (len , mem :: size_of ::< T > ()) ; Ok (value . maybe_uninit) }
};
}
