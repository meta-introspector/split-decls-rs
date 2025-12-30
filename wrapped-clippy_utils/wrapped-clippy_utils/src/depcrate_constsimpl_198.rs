// Generated macro for impl_198 (impl)
macro_rules! Depcrate_constsimpl_198 {
() => {
// Module: crate::consts
// Provides: {"impl_198"}
// Dependencies: {}
impl Hash for Constant { fn hash < H > (& self , state : & mut H) where H : Hasher , { std :: mem :: discriminant (self) . hash (state) ; match * self { Self :: Adt (ref elem) => { elem . hash (state) ; } , Self :: Str (ref s) => { s . hash (state) ; } , Self :: Binary (ref b) => { b . hash (state) ; } , Self :: Char (c) => { c . hash (state) ; } , Self :: Int (i) => { i . hash (state) ; } , Self :: F16 (f) => { f . hash (state) ; } , Self :: F32 (f) => { f64 :: from (f) . to_bits () . hash (state) ; } , Self :: F64 (f) => { f . to_bits () . hash (state) ; } , Self :: F128 (f) => { f . hash (state) ; } , Self :: Bool (b) => { b . hash (state) ; } , Self :: Vec (ref v) | Self :: Tuple (ref v) => { v . hash (state) ; } , Self :: Repeat (ref c , l) => { c . hash (state) ; l . hash (state) ; } , Self :: RawPtr (u) => { u . hash (state) ; } , Self :: Ref (ref r) => { r . hash (state) ; } , Self :: Err => { } , } } }
};
}
