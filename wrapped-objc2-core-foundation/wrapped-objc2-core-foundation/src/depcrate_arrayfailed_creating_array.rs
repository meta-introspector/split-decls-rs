// Generated macro for failed_creating_array (function)
macro_rules! Depcrate_arrayfailed_creating_array {
() => {
// Module: crate::array
// Provides: {"failed_creating_array"}
// Dependencies: {}
# [doc = " Reading the source code:"] # [doc = " <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFArray.c#L391>"] # [doc = " <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFRuntime.c#L323>"] # [doc = ""] # [doc = " It is clear that creating arrays can only realistically fail if allocating"] # [doc = " failed. So we choose to panic/abort in those cases, to roughly match"] # [doc = " `Vec`'s behaviour."] # [cold] fn failed_creating_array (len : CFIndex) -> ! { # [cfg (feature = "alloc")] { use alloc :: alloc :: { handle_alloc_error , Layout } ; use core :: mem :: align_of ; let layout = Layout :: array :: < * const () > (len as usize) . unwrap_or_else (| _ | unsafe { Layout :: from_size_align_unchecked (0 , align_of :: < * const () > ()) }) ; handle_alloc_error (layout) } # [cfg (not (feature = "alloc"))] { panic ! ("failed allocating CFArray holding {len} elements") } }
};
}
