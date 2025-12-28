macro_rules! deps {
    () => {
        LenType!();
        Drain!();
        Vec!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < T , LenT : LenType > Drain < '_ , T , LenT > { # [doc = " Returns the remaining items of this iterator as a slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::{vec, Vec};"] # [doc = ""] # [doc = " let mut vec = Vec::<_, 3>::from_array(['a', 'b', 'c']);"] # [doc = " let mut drain = vec.drain(..);"] # [doc = " assert_eq!(drain.as_slice(), &['a', 'b', 'c']);"] # [doc = " let _ = drain.next().unwrap();"] # [doc = " assert_eq!(drain.as_slice(), &['b', 'c']);"] # [doc = " ```"] # [must_use] pub fn as_slice (& self) -> & [T] { self . iter . as_slice () } }
    };
}

impl_272!()