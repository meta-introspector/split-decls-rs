macro_rules! deps {
    () => {
        Access!();
        DynGuard!();
    };
}

macro_rules! DynAccess {
    () => {
        deps!();
        # [doc = " An object-safe version of the [`Access`] trait."] # [doc = ""] # [doc = " This can be used instead of the [`Access`] trait in case a type erasure is desired. This has"] # [doc = " the effect of performance hit (due to boxing of the result and due to dynamic dispatch), but"] # [doc = " makes certain code simpler and possibly makes the executable smaller."] # [doc = ""] # [doc = " This is automatically implemented for everything that implements [`Access`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use arc_swap::access::{Constant, DynAccess};"] # [doc = ""] # [doc = " fn do_something(value: Box<dyn DynAccess<usize> + Send>) {"] # [doc = "     let v = value.load();"] # [doc = "     println!(\"{}\", *v);"] # [doc = " }"] # [doc = ""] # [doc = " do_something(Box::new(Constant(42)));"] # [doc = " ```"] pub trait DynAccess < T > { # [doc = " The equivalent of [`Access::load`]."] fn load (& self) -> DynGuard < T > ; }
    };
}

DynAccess!();