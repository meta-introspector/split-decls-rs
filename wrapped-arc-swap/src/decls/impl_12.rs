macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T : RefCnt , S : Strategy < T > > Guard < T , S > { # [doc = " Converts it into the held value."] # [doc = ""] # [doc = " This, on occasion, may be a tiny bit faster than cloning the Arc or whatever is being held"] # [doc = " inside."] # [allow (clippy :: wrong_self_convention)] # [inline] pub fn into_inner (lease : Self) -> T { lease . inner . into_inner () } # [doc = " Create a guard for a given value `inner`."] # [doc = ""] # [doc = " This can be useful on occasion to pass a specific object to code that expects or"] # [doc = " wants to store a Guard."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use arc_swap::{ArcSwap, DefaultStrategy, Guard};"] # [doc = " # use std::sync::Arc;"] # [doc = " # let p = ArcSwap::from_pointee(42);"] # [doc = " // Create two guards pointing to the same object"] # [doc = " let g1 = p.load();"] # [doc = " let g2 = Guard::<_, DefaultStrategy>::from_inner(Arc::clone(&*g1));"] # [doc = " # drop(g2);"] # [doc = " ```"] pub fn from_inner (inner : T) -> Self { Guard { inner : S :: Protected :: from_inner (inner) , } } }
    };
}

impl_12!()