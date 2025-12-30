// Generated macro for write_mib (function)
macro_rules! Depcrate_rawwrite_mib {
() => {
// Module: crate::raw
// Provides: {"write_mib"}
// Dependencies: {}
# [doc = " Uses the MIB `mib` as key to the _MALLCTL NAMESPACE_ and writes its `value`."] # [doc = ""] # [doc = " The [`name_to_mib`] API translates a string of the key (e.g. `arenas.nbins`)"] # [doc = " to a `mib` (Management Information Base)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is `unsafe` because it is possible to use it to construct an"] # [doc = " invalid `T`, for example, by passing `T=u8` for a key expecting `bool`. The"] # [doc = " sizes of `bool` and `u8` match, but `bool` cannot represent all values that"] # [doc = " `u8` can."] pub unsafe fn write_mib < T > (mib : & [usize] , mut value : T) -> Result < () > { cvt (tikv_jemalloc_sys :: mallctlbymib (mib . as_ptr () , mib . len () , ptr :: null_mut () , ptr :: null_mut () , & mut value as * mut _ as * mut _ , mem :: size_of :: < T > () ,)) }
};
}
