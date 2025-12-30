// Generated macro for coerce_pointee_expansion (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrocoerce_pointee_expansion {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"coerce_pointee_expansion"}
// Dependencies: {}
# [test] fn coerce_pointee_expansion () { check (r#"
//- minicore: coerce_pointee

use core::marker::CoercePointee;

pub trait Trait<T: ?Sized> {}

#[derive(CoercePointee)]
#[repr(transparent)]
pub struct Foo<'a, T: ?Sized + Trait<U>, #[pointee] U: ?Sized, const N: u32>(T)
where
    U: Trait<U> + ToString;"# , expect ! [[r#"

use core::marker::CoercePointee;

pub trait Trait<T: ?Sized> {}

#[derive(CoercePointee)]
#[repr(transparent)]
pub struct Foo<'a, T: ?Sized + Trait<U>, #[pointee] U: ?Sized, const N: u32>(T)
where
    U: Trait<U> + ToString;
impl <T, U, const N: u32, __S> $crate::ops::DispatchFromDyn<Foo<'a, T, __S, N>> for Foo<T, U, N, > where U: Trait<U> +ToString, T: Trait<__S>, __S: ?Sized, __S: Trait<__S> +ToString, U: ::core::marker::Unsize<__S>, T:?Sized+Trait<U>, U:?Sized, {}
impl <T, U, const N: u32, __S> $crate::ops::CoerceUnsized<Foo<'a, T, __S, N>> for Foo<T, U, N, > where U: Trait<U> +ToString, T: Trait<__S>, __S: ?Sized, __S: Trait<__S> +ToString, U: ::core::marker::Unsize<__S>, T:?Sized+Trait<U>, U:?Sized, {}"#]] ,) ; }
};
}
