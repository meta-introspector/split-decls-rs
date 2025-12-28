macro_rules! deps {
    () => {
        AtomicCell!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T : Copy > AtomicCell < T > { # [doc = " Loads a value from the atomic cell."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::atomic::AtomicCell;"] # [doc = ""] # [doc = " let a = AtomicCell::new(7);"] # [doc = ""] # [doc = " assert_eq!(a.load(), 7);"] # [doc = " ```"] pub fn load (& self) -> T { unsafe { atomic_load (self . as_ptr ()) } } }
    };
}

impl_17!()