macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T : Default > AtomicCell < T > { # [doc = " Takes the value of the atomic cell, leaving `Default::default()` in its place."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::atomic::AtomicCell;"] # [doc = ""] # [doc = " let a = AtomicCell::new(5);"] # [doc = " let five = a.take();"] # [doc = ""] # [doc = " assert_eq!(five, 5);"] # [doc = " assert_eq!(a.into_inner(), 0);"] # [doc = " ```"] pub fn take (& self) -> T { self . swap (Default :: default ()) } }
    };
}

impl_16!();