// Generated macro for update_mib (function)
macro_rules! Depcrate_rawupdate_mib {
() => {
// Module: crate::raw
// Provides: {"update_mib"}
// Dependencies: {}
# [doc = " Uses the MIB `mib` as key to the _MALLCTL NAMESPACE_ and writes its `value`"] # [doc = " returning its previous value."] # [doc = ""] # [doc = " The [`name_to_mib`] API translates a string of the key (e.g. `arenas.nbins`)"] # [doc = " to a `mib` (Management Information Base)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is `unsafe` because it is possible to use it to construct an"] # [doc = " invalid `T`, for example, by passing `T=u8` for a key expecting `bool`. The"] # [doc = " sizes of `bool` and `u8` match, but `bool` cannot represent all values that"] # [doc = " `u8` can."] pub unsafe fn update_mib < T > (mib : & [usize] , mut value : T) -> Result < T > { let mut len = mem :: size_of :: < T > () ; cvt (tikv_jemalloc_sys :: mallctlbymib (mib . as_ptr () , mib . len () , & mut value as * mut _ as * mut _ , & mut len , & mut value as * mut _ as * mut _ , len ,)) ? ; assert_eq ! (len , mem :: size_of ::< T > ()) ; Ok (value) }
};
}
