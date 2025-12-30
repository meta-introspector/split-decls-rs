// Generated macro for Size (type)
macro_rules! Depcrate_header_mapSize {
() => {
// Module: crate::header::map
// Provides: {"Size"}
// Dependencies: {}
# [doc = " Type used for representing the size of a HeaderMap value."] # [doc = ""] # [doc = " 32,768 is more than enough entries for a single header map. Setting this"] # [doc = " limit enables using `u16` to represent all offsets, which takes 2 bytes"] # [doc = " instead of 8 on 64 bit processors."] # [doc = ""] # [doc = " Setting this limit is especially beneficial for `indices`, making it more"] # [doc = " cache friendly. More hash codes can fit in a cache line."] # [doc = ""] # [doc = " You may notice that `u16` may represent more than 32,768 values. This is"] # [doc = " true, but 32,768 should be plenty and it allows us to reserve the top bit"] # [doc = " for future usage."] type Size = u16 ;
};
}
