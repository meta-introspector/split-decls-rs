// Generated macro for read_mib (function)
macro_rules! Depcrate_rawread_mib {
() => {
// Module: crate::raw
// Provides: {"read_mib"}
// Dependencies: {}
# [doc = " Uses the MIB `mib` as key to the _MALLCTL NAMESPACE_ and reads its value."] # [doc = ""] # [doc = " The [`name_to_mib`] API translates a string of the key (e.g. `arenas.nbins`)"] # [doc = " to a `mib` (Management Information Base)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is `unsafe` because it is possible to use it to construct an"] # [doc = " invalid `T`, for example, by passing `T=bool` for a key returning `u8`. The"] # [doc = " sizes of `bool` and `u8` match, but `bool` cannot represent all values that"] # [doc = " `u8` can."] pub unsafe fn read_mib < T : Copy > (mib : & [usize]) -> Result < T > { let mut value = MaybeUninit { init : () } ; let mut len = mem :: size_of :: < T > () ; cvt (tikv_jemalloc_sys :: mallctlbymib (mib . as_ptr () , mib . len () , & mut value . init as * mut _ as * mut _ , & mut len , ptr :: null_mut () , 0 ,)) ? ; assert_eq ! (len , mem :: size_of ::< T > ()) ; Ok (value . maybe_uninit) }
};
}
