// Generated macro for RefCast (trait)
macro_rules! DepcrateRefCast {
() => {
// Module: crate
// Provides: {"RefCast"}
// Dependencies: {}
# [doc = " Safely cast `&T` to `&U` where the struct `U` contains a single field of"] # [doc = " type `T`."] # [doc = ""] # [doc = " ```"] # [doc = " # use ref_cast::RefCast;"] # [doc = " #"] # [doc = " // `&String` can be cast to `&U`."] # [doc = " #[derive(RefCast)]"] # [doc = " #[repr(transparent)]"] # [doc = " struct U(String);"] # [doc = ""] # [doc = " // `&T` can be cast to `&V<T>`."] # [doc = " #[derive(RefCast)]"] # [doc = " #[repr(transparent)]"] # [doc = " struct V<T> {"] # [doc = "     t: T,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " See the [crate-level documentation][crate] for usage examples!"] pub trait RefCast { type From : ? Sized ; fn ref_cast (from : & Self :: From) -> & Self ; fn ref_cast_mut (from : & mut Self :: From) -> & mut Self ; }
};
}
