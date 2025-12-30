// Generated macro for update (function)
macro_rules! Depcrate_rawupdate {
() => {
// Module: crate::raw
// Provides: {"update"}
// Dependencies: {}
# [doc = " Uses the null-terminated string `name` as key to the _MALLCTL NAMESPACE_ and"] # [doc = " writes its `value` returning its previous value."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is `unsafe` because it is possible to use it to construct an"] # [doc = " invalid `T`, for example, by passing `T=u8` for a key expecting `bool`. The"] # [doc = " sizes of `bool` and `u8` match, but `bool` cannot represent all values that"] # [doc = " `u8` can."] pub unsafe fn update < T > (name : & [u8] , mut value : T) -> Result < T > { validate_name (name) ; let mut len = mem :: size_of :: < T > () ; cvt (tikv_jemalloc_sys :: mallctl (name as * const _ as * const c_char , & mut value as * mut _ as * mut _ , & mut len , & mut value as * mut _ as * mut _ , len ,)) ? ; assert_eq ! (len , mem :: size_of ::< T > ()) ; Ok (value) }
};
}
