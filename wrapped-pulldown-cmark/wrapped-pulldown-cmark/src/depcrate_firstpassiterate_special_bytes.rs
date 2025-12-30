// Generated macro for iterate_special_bytes (function)
macro_rules! Depcrate_firstpassiterate_special_bytes {
() => {
// Module: crate::firstpass
// Provides: {"iterate_special_bytes"}
// Dependencies: {}
# [doc = " This function walks the byte slices from the given index and"] # [doc = " calls the callback function on all bytes (and their indices) that are in the following set:"] # [doc = " `` ` ``, `\\`, `&`, `*`, `_`, `~`, `!`, `<`, `[`, `]`, `|`, `\\r`, `\\n`"] # [doc = " It is guaranteed not call the callback on other bytes."] # [doc = " Whenever `callback(ix, byte)` returns a `ContinueAndSkip(n)` value, the callback"] # [doc = " will not be called with an index that is less than `ix + n + 1`."] # [doc = " When the callback returns a `BreakAtWith(end_ix, opt+val)`, no more callbacks will be"] # [doc = " called and the function returns immediately with the return value `(end_ix, opt_val)`."] # [doc = " If `BreakAtWith(..)` is never returned, this function will return the first"] # [doc = " index that is outside the byteslice bound and a `None` value."] fn iterate_special_bytes < F , T > (lut : & LookupTable , bytes : & [u8] , ix : usize , callback : F ,) -> (usize , Option < T >) where F : FnMut (usize , u8) -> LoopInstruction < Option < T > > , { # [cfg (all (target_arch = "x86_64" , feature = "simd"))] { simd :: iterate_special_bytes (lut , bytes , ix , callback) } # [cfg (not (all (target_arch = "x86_64" , feature = "simd")))] { scalar_iterate_special_bytes (lut , bytes , ix , callback) } }
};
}
