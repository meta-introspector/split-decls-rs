// Generated macro for IntoFuture (trait)
macro_rules! DepcrateIntoFuture {
() => {
// Module: crate
// Provides: {"IntoFuture"}
// Dependencies: {}
# [doc = " Class of types which can be converted themselves into a future."] # [doc = ""] # [doc = " This trait is very similar to the `IntoIterator` trait and is intended to be"] # [doc = " used in a very similar fashion."] pub trait IntoFuture : Send + 'static { # [doc = " The future that this type can be converted into."] type Future : Future < Item = Self :: Item , Error = Self :: Error > ; # [doc = " The item that the future may resolve with."] type Item : Send + 'static ; # [doc = " The error that the future may resolve with."] type Error : Send + 'static ; # [doc = " Consumes this object and produces a future."] fn into_future (self) -> Self :: Future ; }
};
}
