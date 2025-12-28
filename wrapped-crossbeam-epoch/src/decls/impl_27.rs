macro_rules! deps {
    () => {
        Pointable!();
        Owned!();
        Atomic!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > From < Owned < T > > for Atomic < T > { # [doc = " Returns a new atomic pointer pointing to `owned`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::{Atomic, Owned};"] # [doc = ""] # [doc = " let a = Atomic::<i32>::from(Owned::new(1234));"] # [doc = " # unsafe { drop(a.into_owned()); } // avoid leak"] # [doc = " ```"] fn from (owned : Owned < T >) -> Self { let data = owned . data ; mem :: forget (owned) ; Self :: from_ptr (data) } }
    };
}

impl_27!()