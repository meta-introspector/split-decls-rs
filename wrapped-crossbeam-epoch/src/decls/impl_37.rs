macro_rules! deps {
    () => {
        Owned!();
        Shared!();
        Pointable!();
        Guard!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Owned < T > { # [doc = " Allocates `value` on the heap and returns a new owned pointer pointing to it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Owned;"] # [doc = ""] # [doc = " let o = Owned::<i32>::init(1234);"] # [doc = " ```"] pub fn init (init : T :: Init) -> Self { unsafe { Self :: from_ptr (T :: init (init)) } } # [doc = " Converts the owned pointer into a [`Shared`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::{self as epoch, Owned};"] # [doc = ""] # [doc = " let o = Owned::new(1234);"] # [doc = " let guard = &epoch::pin();"] # [doc = " let p = o.into_shared(guard);"] # [doc = " # unsafe { drop(p.into_owned()); } // avoid leak"] # [doc = " ```"] # [allow (clippy :: needless_lifetimes)] pub fn into_shared < 'g > (self , _ : & 'g Guard) -> Shared < 'g , T > { unsafe { Shared :: from_ptr (self . into_ptr ()) } } # [doc = " Returns the tag stored within the pointer."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Owned;"] # [doc = ""] # [doc = " assert_eq!(Owned::new(1234).tag(), 0);"] # [doc = " ```"] pub fn tag (& self) -> usize { let (_ , tag) = decompose_tag :: < T > (self . data) ; tag } # [doc = " Returns the same pointer, but tagged with `tag`. `tag` is truncated to be fit into the"] # [doc = " unused bits of the pointer to `T`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_epoch::Owned;"] # [doc = ""] # [doc = " let o = Owned::new(0u64);"] # [doc = " assert_eq!(o.tag(), 0);"] # [doc = " let o = o.with_tag(2);"] # [doc = " assert_eq!(o.tag(), 2);"] # [doc = " ```"] pub fn with_tag (self , tag : usize) -> Self { let data = self . into_ptr () ; unsafe { Self :: from_ptr (compose_tag :: < T > (data , tag)) } } }
    };
}

impl_37!()