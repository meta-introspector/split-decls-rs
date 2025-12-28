macro_rules! deps {
    () => {
        Access!();
        Map!();
        DynAccess!();
    };
}

macro_rules! AccessConvert {
    () => {
        deps!();
        # [doc = " [DynAccess] to [Access] wrapper."] # [doc = ""] # [doc = " In previous versions, `Box<dyn DynAccess>` didn't implement [Access], to use inside [Map] one"] # [doc = " could use this wrapper. Since then, a way was found to solve it. In most cases, this wrapper is"] # [doc = " no longer necessary."] # [doc = ""] # [doc = " This is left in place for two reasons:"] # [doc = " * Backwards compatibility."] # [doc = " * Corner-cases not covered by the found solution. For example, trait inheritance in the form of"] # [doc = "   `Box<dyn SomeTrait>` where `SomeTrait: Access` doesn't work out of the box and still needs"] # [doc = "   this wrapper."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " The example is for the simple case (which is no longer needed, but may help as an inspiration)."] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " use arc_swap::ArcSwap;"] # [doc = " use arc_swap::access::{AccessConvert, DynAccess, Map};"] # [doc = ""] # [doc = " struct Inner {"] # [doc = "     val: usize,"] # [doc = " }"] # [doc = ""] # [doc = " struct Middle {"] # [doc = "     inner: Inner,"] # [doc = " }"] # [doc = ""] # [doc = " struct Outer {"] # [doc = "     middle: Middle,"] # [doc = " }"] # [doc = ""] # [doc = " let outer = Arc::new(ArcSwap::from_pointee(Outer {"] # [doc = "     middle: Middle {"] # [doc = "         inner: Inner {"] # [doc = "             val: 42,"] # [doc = "         }"] # [doc = "     }"] # [doc = " }));"] # [doc = ""] # [doc = " let middle: Arc<dyn DynAccess<Middle>> ="] # [doc = "     Arc::new(Map::new(outer, |outer: &Outer| &outer.middle));"] # [doc = " let inner: Arc<dyn DynAccess<Inner>> ="] # [doc = "     Arc::new(Map::new(AccessConvert(middle), |middle: &Middle| &middle.inner));"] # [doc = " let guard = inner.load();"] # [doc = " assert_eq!(42, guard.val);"] # [doc = " ```"] pub struct AccessConvert < D > (pub D) ;
    };
}

AccessConvert!();