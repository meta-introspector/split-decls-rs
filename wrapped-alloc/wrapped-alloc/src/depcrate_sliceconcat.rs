// Generated macro for Concat (trait)
macro_rules! Depcrate_sliceConcat {
() => {
// Module: crate::slice
// Provides: {"Concat"}
// Dependencies: {}
# [doc = " Helper trait for [`[T]::concat`](slice::concat)."] # [doc = ""] # [doc = " Note: the `Item` type parameter is not used in this trait,"] # [doc = " but it allows impls to be more generic."] # [doc = " Without it, we get this error:"] # [doc = ""] # [doc = " ```error"] # [doc = " error[E0207]: the type parameter `T` is not constrained by the impl trait, self type, or predica"] # [doc = "    --> library/alloc/src/slice.rs:608:6"] # [doc = "     |"] # [doc = " 608 | impl<T: Clone, V: Borrow<[T]>> Concat for [V] {"] # [doc = "     |      ^ unconstrained type parameter"] # [doc = " ```"] # [doc = ""] # [doc = " This is because there could exist `V` types with multiple `Borrow<[_]>` impls,"] # [doc = " such that multiple `T` types would apply:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[allow(dead_code)]"] # [doc = " pub struct Foo(Vec<u32>, Vec<String>);"] # [doc = ""] # [doc = " impl std::borrow::Borrow<[u32]> for Foo {"] # [doc = "     fn borrow(&self) -> &[u32] { &self.0 }"] # [doc = " }"] # [doc = ""] # [doc = " impl std::borrow::Borrow<[String]> for Foo {"] # [doc = "     fn borrow(&self) -> &[String] { &self.1 }"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "slice_concat_trait" , issue = "27747")] pub trait Concat < Item : ? Sized > { # [unstable (feature = "slice_concat_trait" , issue = "27747")] # [doc = " The resulting type after concatenation"] type Output ; # [doc = " Implementation of [`[T]::concat`](slice::concat)"] # [unstable (feature = "slice_concat_trait" , issue = "27747")] fn concat (slice : & Self) -> Self :: Output ; }
};
}
